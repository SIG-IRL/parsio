use pyo3::prelude::*;
use std::collections::HashSet;
use regex::Regex;

#[pyclass]
#[allow(dead_code)] 
pub struct Tokenizer {
    join_verb_parts: bool,
    split_zwnj: bool,
    separate_emoji: bool,
    extract_hashtags: bool,
    preserve_emails: bool,
    convert_numbers: bool,
    preserve_links: bool,
    after_verbs: HashSet<String>,
    before_verbs: HashSet<String>,
}

#[pymethods]
impl Tokenizer {
    #[new]
    pub fn new(
        join_verb_parts: bool,
        split_zwnj: bool,
        separate_emoji: bool,
        extract_hashtags: bool,
        preserve_emails: bool,
        convert_numbers: bool,
        preserve_links: bool,
    ) -> Self {
        let after_verbs = [
            "است", "بود", "شده_است", "می‌شود", "خواهد_شد",
        ].iter().map(|s| s.to_string()).collect();
        let before_verbs = [
            "می", "نمی", "خواهد", "نخواهد",
        ].iter().map(|s| s.to_string()).collect();

        Tokenizer {
            join_verb_parts,
            split_zwnj,
            separate_emoji,
            extract_hashtags,
            preserve_emails,
            convert_numbers,
            preserve_links,
            after_verbs,
            before_verbs,
        }
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut working_text = text.to_string();

        // Step 1: Preprocessing with unique markers
        if self.convert_numbers {
            working_text = self.convert_persian_numbers(&working_text);
        }
        if self.preserve_emails {
            let email_pattern = Regex::new(r"[a-zA-Z0-9._+-]+@([a-zA-Z0-9-]+\.)+[A-Za-z]{2,}").unwrap();
            working_text = email_pattern.replace_all(&working_text, "\x01$0\x01").to_string(); // Use SOH (Start of Heading) as marker
        }
        if self.preserve_links {
            let link_pattern = Regex::new(r"(https?://)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}[-\w@:%_.+/~#?=&]*").unwrap();
            working_text = link_pattern.replace_all(&working_text, "\x01$0\x01").to_string(); // Use SOH as marker
        }
        if self.extract_hashtags {
            let hashtag_pattern = Regex::new(r"#\S+").unwrap();
            working_text = hashtag_pattern.replace_all(&working_text, |caps: &regex::Captures| {
                let hashtag = &caps[0];
                let parts: Vec<String> = hashtag[1..] // Skip '#'
                    .split('_')
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();
                format!("# {}", parts.join(" "))
            }).to_string();
        }
        if self.separate_emoji {
            let emoji_pattern = Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}]").unwrap();
            working_text = emoji_pattern.replace_all(&working_text, " $0 ").to_string();
        }

        // Step 2: Split into segments, preserving marked tokens
        let segments: Vec<String> = working_text
            .split('\x01') // Split on SOH markers
            .map(String::from)
            .collect();

        // Step 3: Process each segment
        let mut tokens = Vec::new();
        for (i, segment) in segments.into_iter().enumerate() {
            if i % 2 == 1 {
                // Odd indices are protected tokens (emails, links)
                tokens.push(segment);
            } else {
                // Even indices are regular text
                let sub_tokens: Vec<String> = segment
                    .split_whitespace()
                    .flat_map(|token| {
                        if self.split_zwnj {
                            token.split('\u{200c}').map(String::from).collect::<Vec<_>>()
                        } else {
                            vec![token.to_string()]
                        }
                    })
                    .filter(|t| !t.is_empty())
                    .collect();
                for token in sub_tokens {
                    if let Some((prefix, suffix)) = self.split_mixed(&token) {
                        tokens.push(prefix);
                        tokens.push(suffix);
                    } else {
                        tokens.push(token);
                    }
                }
            }
        }

        // Step 4: Join verb parts
        if self.join_verb_parts {
            tokens = self.join_verb_parts(tokens);
        }

        tokens
    }

    fn convert_persian_numbers(&self, text: &str) -> String {
        let persian_to_english: Vec<(char, char)> = vec![
            ('۰', '0'), ('۱', '1'), ('۲', '2'), ('۳', '3'), ('۴', '4'),
            ('۵', '5'), ('۶', '6'), ('۷', '7'), ('۸', '8'), ('۹', '9'),
            ('٫', '.'),
        ];
        let mut result = text.to_string();
        for (persian, english) in persian_to_english {
            result = result.replace(persian, &english.to_string());
        }
        result
    }

    fn split_mixed(&self, token: &str) -> Option<(String, String)> {
        let chars: Vec<char> = token.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if c.is_ascii_alphabetic() && i > 0 && chars[i - 1].is_alphabetic() && !chars[i - 1].is_ascii() {
                return Some((
                    chars[..i].iter().collect(),
                    chars[i..].iter().collect()
                ));
            }
            if !c.is_ascii() && i > 0 && chars[i - 1].is_ascii_alphabetic() {
                return Some((
                    chars[..i].iter().collect(),
                    chars[i..].iter().collect()
                ));
            }
        }
        None
    }

    fn join_verb_parts(&self, tokens: Vec<String>) -> Vec<String> {
        if tokens.len() <= 1 {
            return tokens;
        }

        let mut result = Vec::new();
        let mut current = String::new();

        for token in tokens.into_iter().rev() {
            if self.before_verbs.contains(&token) {
                current = format!("{}_{}", token, current);
            } else if !current.is_empty() && self.after_verbs.contains(&token) {
                current = format!("{}_{}", token, current);
            } else {
                if !current.is_empty() {
                    result.push(current);
                }
                current = token;
            }
        }
        if !current.is_empty() {
            result.push(current);
        }

        result.into_iter().rev().collect()
    }
}
