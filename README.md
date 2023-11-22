# Jippety-Kohmmit

A Rust CLI tool for generating conventional commit messages using OpenAI's GPT-4 turbo model.

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/username/jippety-kohmmit.git
cd jippety-kohmmit
cargo build --release
```

## Configuration

Place a `config.toml` in your home directory under `~/.config/jippety-kohmmit/` with the following structure:

```toml
openai_api_key = "sk-yourapikey..."
```

## Usage

Run the binary, and it will calculate diffs on staged files, pass the diff to OpenAI, and generate a commit message.

Make sure to stage the files you want to include in your commit: