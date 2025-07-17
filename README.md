# R2D2

R2D2 is a terminal-based AI assistant written in Rust. It leverages the [ollama-rs](https://github.com/pepperoni21/ollama-rs) library to interact with local LLMs and provides a set of tools for shell command execution, file system interaction, and more. R2D2 is designed to be your smart terminal agent, able to execute commands, read files, and assist with various tasks directly from your CLI.

## Features
- Interact with local LLMs using Ollama models
- Execute shell commands safely (with user confirmation)
- Read and process files from the file system
- Extensible tool system for new capabilities
- Always responds in Italian

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/CiroBurro/r2d2.git
   cd r2d2
   ```
2. **Build with Cargo:**
   ```sh
   cargo build --release
   ```
3. **(Optional) Move the executable to a directory in your $PATH:**
   ```sh
   sudo mv target/release/r2d2 /usr/local/bin/
   ```
   This allows you to run `r2d2` from anywhere in the terminal.
4. **Run:**
   ```sh
   cargo run --release -- --model <model-name>
   ```
   Or, if you moved the executable:
   ```sh
   r2d2 --model <model-name>
   ```
   Replace `<model-name>` with the Ollama model you want to use (default: `qwen3:8b`).

## Usage

R2D2 is a CLI tool. Example usage using the default model `qwen3:8b`:

```sh
r2d2
```


Or, if you did not move the executable:

```sh
cargo run --release
```


You can also specify a custom system prompt file and a different model:

```sh
r2d2 --model deepseek-r1:8b --prompt /path/to/prompt.txt
```

Or:

```sh
cargo run --release -- --model deepseek-r1:8b --prompt /path/to/prompt.txt
```

### Command-line options
- `--model`, `-m`: Choose the Ollama model to use (default: `qwen3:8b`)
- `--prompt`, `-p`: Path to a custom system prompt file

## Notes
- R2D2 will always ask for confirmation before executing shell commands.
- All responses and explanations are provided in Italian with the default prompt
- The tool system is extensible; you can add new tools in `src/tools.rs`.
- Requires Rust 2021+ and a working Ollama setup for local LLMs.

