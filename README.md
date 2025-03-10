# Parsio: A Persian NLP Toolkit
*Parsio is still under development*

**Parsio** is an advanced, open-source Natural Language Processing (NLP) toolkit designed for Persian (Farsi) and other Iranian languages. Built with performance and scalability in mind, it combines a high-speed Rust core with an intuitive Python interface, making it ideal for researchers, developers, and Persian-speaking communities worldwide.

Developed under the **SIG-IRL (Special Interest Group for Iranian Languages)**, an ACL-affiliated initiative, Parsio aims to address the lack of robust, universal NLP tools for Persian—a language with unique linguistic challenges and a growing digital presence.

---

## Features
- **Modular Design**: Extensible components for tokenization, morphology, and more.
- **High Performance**: Rust-powered core for speed and reliability.
- **Python-Friendly**: Seamless integration with Python via `pip install parsio`.
- **Persian-Focused**: Tailored to handle Persian script, ZWNJ, and morphological quirks.

## Installation
```bash
pip install parsio
```
See `docs/installation.rst` for detailed instructions.

## Quick Start
```python
from parsio import Tokenizer

tokenizer = Tokenizer(split_zwnj=True)
text = "سلام می‌روم به دنیا"
tokens = tokenizer.tokenize(text)
print(tokens)  # ['سلام', 'می', 'روم', 'به', 'جهان']
```
More examples in `examples/`.

## About SIG-IRL
The **Special Interest Group for Iranian Languages (SIG-IRL)**, under the Association for Computational Linguistics (ACL), is a community of students, professionals, and researchers dedicated to advancing NLP for Iranian languages. Parsio is our flagship project, aimed at filling the gap in Persian NLP toolkits.
- **Mission**: Build open-source tools, foster collaboration, and support Persian-speaking developers.
- **Get Involved**: Join us! See `CONTRIBUTING.md`.

## Project Structure
- `core/`: Rust implementation of NLP modules.
- `bindings/`: Python bindings for easy access.
- `docs/`: User and developer documentation.
- `examples/`: Ready-to-run scripts.
- `resources/`: Datasets and models for Persian NLP.

## Contributing
We welcome contributions! Whether you’re a coder, linguist, or Persian speaker, there’s a place for you:
1. Check out `CONTRIBUTING.md`.
2. File issues or submit PRs on GitHub.
3. Join SIG-IRL discussions (details coming soon).

## License
Parsio is released under the MIT License.

## Contact
- GitHub: sig-irl/parsio
- Twitter: Follow updates at @SIG_IRL (TBD)
- Email: sig-irl-contact@example.com (TBD)

Built with ❤️ by the SIG-IRL community.
