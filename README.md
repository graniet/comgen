<img src="https://i.postimg.cc/SKST2N1J/image-99.jpg" width="300" alt="logo">


ComGen is a command-line tool that automatically generates Git commit messages using Large Language Models (LLMs). It analyzes modified files in your Git repository and creates relevant commit messages based on code differences.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

[![asciicast](https://asciinema.org/a/jx1V2DqcGIhgXATjQzGVUFANy.svg)](https://asciinema.org/a/jx1V2DqcGIhgXATjQzGVUFANy)

## Features ✨

- 🔍 Analyzes modified files in your Git repository
- 🤖 Uses LLMs to generate intelligent commit messages
- 🔧 Supports multiple LLM providers (OpenAI, Anthropic, Ollama)
- 🛡️ Performs security audits on code changes
- 📝 Allows customization of commit message templates
- 🌐 Works on Windows, macOS and Linux

## Configuration 🛠️

ComGen uses two main configuration files:

### Main Configuration (`config.yaml`)

The main configuration allows you to define:

- `provider`: The LLM provider ("openai", "anthropic", "ollama")
- `model`: The model to use (e.g., "gpt-4")
- `base_prompt`: The base prompt for generation
- `templates`: Commit template configuration
- API keys for different providers

### Commit Template (`comgen.template`)

The `comgen.template` file defines the structure of commit messages, it can be used at the root of a project to define a particular standard.

- `commit_types`: List of allowed commit types (feat, fix, docs, etc.)
- `output_format`: Output format configuration
  - `template`: Message structure "<type>[optional scope]: <description>"
  - `max_length`: Maximum message length (100 characters)
  - `examples`: Examples of valid commit messages