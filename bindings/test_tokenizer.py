from parsio import Tokenizer

tokenizer = Tokenizer(
    join_verb_parts=True,
    split_zwnj=True,
    extract_hashtags=True,
    preserve_emails=True,
    convert_numbers=True,
    preserve_links=True
)

text = "می\u200cروم به #فردا_امروز ali@example.com ۱۲۳.۴ https://example.com 😂"
tokens = tokenizer.tokenize(text)
print(tokens)
