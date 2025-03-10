# Contributing to Parsio

Welcome to **Parsio**, the flagship open-source Persian NLP toolkit developed under the **SIG-IRL (Special Interest Group for Iranian Languages)**, an ACL-affiliated initiative. Whether you’re a programmer, linguist, native Persian speaker, student, researcher, or just passionate about advancing Persian language technology, we’re thrilled to have you here! Your contributions—big or small—help us build a powerful, universal toolkit for Persian and other Iranian languages.

This guide is designed to be your roadmap. It covers *what* you can contribute, *why* it matters, *where* to start, *how* to get involved, and answers common questions you might have.

---

## Why Contribute?
Parsio addresses a critical gap: Persian lacks a robust, scalable NLP toolkit despite its rich linguistic heritage and growing digital presence. By contributing, you:
- Empower Persian-speaking communities with better language tools.
- Advance research and innovation in low-resource NLP.
- Gain hands-on experience with Rust, Python, and cutting-edge NLP.
- Join a global community under SIG-IRL, part of the prestigious ACL.

No matter your background, your input shapes the future of this project.

---

## Who Can Contribute?
Everyone! We welcome:
- **Coders**: Rust or Python developers to build and optimize modules.
- **Linguists**: Experts in Persian or Iranian languages to guide feature development.
- **Native Speakers**: Persian speakers to test tools, provide feedback, or annotate data.
- **Students**: Learners eager to dive into NLP, Rust, or open-source projects.
- **Professionals**: Industry experts to contribute advanced algorithms or use cases.
- **Documentation Writers**: Communicators to improve tutorials and guides.
- **Designers**: Creatives to enhance visuals or UI (e.g., for future GUI tools).

No prior experience? No problem—we’ll help you get started!

---

## What Can You Contribute?
Parsio is modular and multifaceted, so there’s a role for every skill set. Here are some ideas:

### 1. Code Contributions
- **Rust Core**: Enhance `core/modules/` (e.g., tokenizer, morphology, NER).
- **Python Bindings**: Improve `bindings/src/parsio/` for a seamless user experience.
- **Performance**: Optimize algorithms or add parallel processing.
- **Tests**: Write unit tests in `core/tests/` or integration tests in `bindings/src/tests/`.
- **New Features**: Propose and implement new NLP modules (e.g., sentiment analysis).

### 2. Linguistic Expertise
- **Persian Rules**: Define tokenization or morphology rules for Persian quirks (e.g., ZWNJ, light verbs).
- **Dialects**: Add support for Persian dialects (e.g., Tehrani, Tajik) or other Iranian languages (e.g., Kurdish).
- **Validation**: Test outputs against linguistic accuracy.

### 3. Data Contributions
- **Datasets**: Curate Persian text corpora for `resources/datasets/` (e.g., news, social media).
- **Annotation**: Label data for tasks like NER or sentiment analysis.
- **Models**: Train and share pretrained embeddings or models in `resources/models/`.

### 4. Documentation
- **User Guides**: Write tutorials in `docs/tutorials/` (English or Persian).
- **API Docs**: Document Rust and Python code in `docs/api/`.
- **Dev Docs**: Explain architecture or workflows in `docs/dev/`.

### 5. Community & Outreach
- **Examples**: Create demos for `examples/` to showcase Parsio.
- **Bug Reports**: File issues with detailed repro steps.
- **Feedback**: Suggest improvements via discussions or issues.
- **Events**: Organize SIG-IRL hackathons or workshops (contact us!).

### 6. Other Ideas
- Fix typos, improve README, design logos, translate docs, or propose wild new features—we’re open to it all!

---

## How to Get Started
### Prerequisites
- **Git**: For cloning and submitting changes.
- **Rust**: For `core/` development (install via [rustup.rs](https://rustup.rs/)).
- **Python**: For `bindings/` and testing (3.8+ recommended).
- **Text Editor**: EMACS, VIM, VSCode, IntelliJ, or any IDE you prefer.
- **Optional**: Familiarity with NLP, Persian linguistics, or open-source workflows.

### Step-by-Step Guide
1. **Fork the Repo**:
   - Click "Fork" on [github.com/sig-irl/parsio](https://github.com/sig-irl/parsio).
   - Clone your fork: `git clone https://github.com/your-username/parsio.git`.

2. **Set Up Locally**:
   - Rust: `cd core && cargo build`.
   - Python: `cd bindings && pip install -e .` (uses `maturin` to link Rust).
   - Verify: Run `python -c "import parsio; print(parsio.__version__)"`.

3. **Find Something to Work On**:
   - Browse [Issues](https://github.com/sig-irl/parsio/issues) tagged `good-first-issue` or `help-wanted`.
   - Propose your own idea via a new issue.

4. **Make Changes**:
   - Create a branch: `git checkout -b feat/your-feature-name`.
   - Code, test, and document your work (see style guides below).

5. **Submit a Pull Request (PR)**:
   - Push your branch: `git push origin feat/your-feature-name`.
   - Open a PR on GitHub, linking to the issue (if applicable).
   - Describe *what* you did, *why*, and *how* it works.

6. **Review Process**:
   - SIG-IRL maintainers will review your PR.
   - Expect feedback—collaboration is key!
   - Once approved, it’s merged into `dev` and eventually `main`.

---

## Development Guidelines
### Code Style
- **Rust**: Follow `rustfmt` (`cargo fmt`) and `clippy` (`cargo clippy`) standards.
- **Python**: Adhere to PEP 8 (use `black` and `flake8`).
- **Naming**: Use clear, descriptive names (e.g., `extract_ner`).

### Testing
- Write tests for every change:
  - Rust: Add to `core/tests/`.
  - Python: Add to `bindings/src/tests/`.
- Run locally: `cargo test` and `pytest`.

### Documentation
- Update `docs/` for new features or changes.
- Use docstrings in Python and `///` comments in Rust.

### Commit Messages
- Use semantic commits: `feat: add tokenizer`, `fix: handle ZWNJ`, `docs: update tutorial`.
- Keep messages concise yet informative.

---

## Common Questions & Answers
**Q: I’m new to open source—where do I start?**
- Start with a `good-first-issue` or ask in Discussions for guidance. We’ll pair you with a mentor if needed!

**Q: I don’t know Rust or Python—can I still help?**
- Yes! Contribute datasets, test as a Persian speaker, or improve docs. We’ll teach you coding if you’re interested.

**Q: What if my idea is too big?**
- Break it into smaller issues or discuss it with us first. No idea is too ambitious!

**Q: How do I test Persian-specific features?**
- Use `resources/datasets/` or your own Persian text. Share edge cases with us.

**Q: Can I contribute in Persian?**
- Absolutely! Tutorials, comments, or datasets in Persian are welcome.

**Q: What if I mess up?**
- You won’t! Mistakes are part of learning, and our CI catches errors. We’re here to help.

---

## Tools & Resources
- **GitHub**: [sig-irl/parsio](https://github.com/sig-irl/parsio)
- **Docs**: [docs/](docs/) or hosted on ReadTheDocs (TBD).
- **Chat**: Join our Slack/Discord (TBD—contact us for now).
- **Persian NLP Basics**: See [docs/tutorials/persian_nlp.rst](docs/tutorials/persian_nlp.rst).
- **Rust Help**: [Rust Book](https://doc.rust-lang.org/book/).
- **Python Help**: [Python Docs](https://docs.python.org/3/).

---

## Community Standards
We follow the [Code of Conduct](CODE_OF_CONDUCT.md) to ensure a respectful, inclusive environment. Report any concerns to sig-irl-contact@example.com (TBD).

---

## Recognition
- Your name goes in `CONTRIBUTORS.md` (opt-in).
- Major contributors may be featured at SIG-IRL events or workshops.
- Students: This is a great portfolio piece or research opportunity!

---

## Contact Us
- **Issues**: File bugs or ideas on GitHub.
- **Discussions**: Join GitHub Discussions for brainstorming.
- **Email**: sig-irl-contact@example.com (TBD).
- **Twitter**: Follow [@SIG_IRL](https://twitter.com/SIG_IRL) (TBD).

---

## Final Note
Parsio is a community effort under SIG-IRL, and every contribution moves us closer to a world-class Persian NLP toolkit. Whether you fix a typo or build a new module, you’re part of something big. Let’s do this together!

Happy contributing! 🚀
