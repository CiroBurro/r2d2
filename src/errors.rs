// Errors module defines custom errors

// Necessary imports
use thiserror::Error;

// General errors enum
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Tool failed: {tool_name};\nError: {error}")]
    ToolCalling { tool_name: String, error: String },
    #[error("Figlet failed")]
    Figlet,
}

// Cli usage errors
#[derive(Error, Debug, Clone)]
pub enum CliError {
    #[error("Specify an existent ollama model with flag '- m' or use default (qwen3:8b)")]
    ModelError,

    #[error("Custom prompt error: {error}\nSpecify a text file with a custom prompt with flag '-p' or use default (italian language)")]
    PromptError { error: String },
}
