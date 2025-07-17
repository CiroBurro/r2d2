// Cli module defines the cli args of the program

// Necessary imports
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "R2D2", about = "CLI ai agent at yout service")]
pub struct Args {
    #[structopt(
        short,
        long,
        default_value = "qwen3:8b",
        about = "Choose the ollama model to use"
    )]
    pub model: String, // Custom model option

    #[structopt(parse(from_os_str), short, long, about = "Use a custom system prompt")]
    pub prompt: Option<PathBuf>, // Custom prompt option
}
