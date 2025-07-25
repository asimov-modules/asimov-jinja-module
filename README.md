# ASIMOV Jinja Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-jinja-module.svg)](https://crates.io/crates/asimov-jinja-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-jinja-module.svg)](https://pypi.org/project/asimov-jinja-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-jinja-module.svg)](https://rubygems.org/gems/asimov-jinja-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-jinja-module.svg)](https://npmjs.com/package/asimov-jinja-module)
[![Documentation](https://docs.rs/asimov-jinja-module/badge.svg)](https://docs.rs/asimov-jinja-module)

[ASIMOV] module for prompt templating using the [Jinja] templating language.

## ✨ Features

- Renders JSON inputs using the [MiniJinja] templating engine.
- Loads environment variables from `.env` (aka dotenv) files.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install jinja -v
```

### Installation from PyPI

```bash
pip install -U asimov-jinja-module
```

### Installation from RubyGems

```bash
gem install asimov-jinja-module
```

### Installation from NPM

```bash
npm install -g asimov-jinja-module
```

### Installation from Source Code

```bash
cargo install asimov-jinja-module
```

## 👉 Examples

### JSON-to-Markdown Rendering

```bash
asimov-jinja-runner template.j2 < input.json > output.md
```

## ⚙ Configuration

This module requires no configuration.

## 📚 Reference

### Installed Binaries

- `asimov-jinja-runner`: renders JSON from standard input to standard output

### `asimov-jinja-runner`

```
asimov-jinja-runner

Usage: asimov-jinja-runner [OPTIONS] [TEMPLATES]...

Arguments:
  [TEMPLATES]...  The `.j2` template files to render

Options:
  -d, --debug       Enable debugging output
      --license     Show license information
  -v, --verbose...  Enable verbose output (may be repeated for more verbosity)
  -V, --version     Print version information
  -h, --help        Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-jinja-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-jinja-module&text=asimov-jinja-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-jinja-module&title=asimov-jinja-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-jinja-module&t=asimov-jinja-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-jinja-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-jinja-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[Jinja]: https://en.wikipedia.org/wiki/Jinja_(template_engine)
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[Mbox]: https://en.wikipedia.org/wiki/Mbox
[MiniJinja]: https://crates.io/crates/minijinja
[NPM]: https:/npmjs.org
[Python]: https://python.org
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
[jq]: https://en.wikipedia.org/wiki/Jq_(programming_language)
