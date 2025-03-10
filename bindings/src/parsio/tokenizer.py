from typing import List
from parsio._parsio import Tokenizer as RustTokenizer 

class Tokenizer:
    """A Persian text tokenizer with configurable options.

    Args:
        join_verb_parts (bool, optional): Join verb parts (e.g., "گفته_شده_است"). Defaults to False.
        split_zwnj (bool, optional): Split on zero-width non-joiner (ZWNJ). Defaults to True.
        separate_emoji (bool, optional): Separate emojis as individual tokens. Defaults to False.
        extract_hashtags (bool, optional): Extract hashtags (e.g., "#فردا_امروز" -> "#", "فردا", "امروز"). Defaults to False.
        preserve_emails (bool, optional): Preserve email addresses as single tokens. Defaults to False.
        convert_numbers (bool, optional): Convert Persian/Arabic numbers to English. Defaults to False.
        preserve_links (bool, optional): Preserve URLs as single tokens. Defaults to False.
    """
    def __init__(
        self,
        join_verb_parts: bool = False,
        split_zwnj: bool = True,
        separate_emoji: bool = False,
        extract_hashtags: bool = False,
        preserve_emails: bool = False,
        convert_numbers: bool = False,
        preserve_links: bool = False
    ):
        self._tokenizer = RustTokenizer(
            join_verb_parts,
            split_zwnj,
            separate_emoji,
            extract_hashtags,
            preserve_emails,
            convert_numbers,
            preserve_links
        )

    def tokenize(self, text: str) -> List[str]:
        """Tokenize the input text into a list of tokens.

        Args:
            text (str): The input text to tokenize.

        Returns:
            List[str]: A list of tokenized strings.
        """
        return self._tokenizer.tokenize(text)
