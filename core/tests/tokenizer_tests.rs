#[cfg(test)]
mod tests {
    use parsio_core::modules::tokenizer::Tokenizer;

    #[test]
    fn test_basic_tokenize() {
        let tokenizer = Tokenizer::new(false, false, false, false, false, false, false);
        let tokens = tokenizer.tokenize("سلام دنیا");
        assert_eq!(tokens, vec!["سلام", "جهان"]);
    }

    #[test]
    fn test_zwnj_split() {
        let tokenizer = Tokenizer::new(false, true, false, false, false, false, false);
        let tokens = tokenizer.tokenize("می\u{200c}روم");
        assert_eq!(tokens, vec!["می", "روم"]);
    }

    #[test]
    fn test_verb_joining() {
        let tokenizer = Tokenizer::new(true, false, false, false, false, false, false);
        let tokens = tokenizer.tokenize("گفته شده است");
        assert_eq!(tokens, vec!["گفته_شده_است"]);
    }

    #[test]
    fn test_mixed_language() {
        let tokenizer = Tokenizer::new(false, false, false, false, false, false, false);
        let tokens = tokenizer.tokenize("helloسلام");
        assert_eq!(tokens, vec!["hello", "سلام"]);
    }

    #[test]
    fn test_emoji_separation() {
        let tokenizer = Tokenizer::new(false, false, true, false, false, false, false);
        let tokens = tokenizer.tokenize("خوبم😂");
        assert_eq!(tokens, vec!["خوبم", "😂"]);
    }
}
