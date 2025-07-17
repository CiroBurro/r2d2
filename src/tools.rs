// Tools module defines all the functions avaible for the model

// Necessary imports
use crate::errors::Error;
use std::{fs::File, io::Read, process::Command};

/// Exec a CLI command to interact with the shell
///
/// Args:
///     - command: command to execute
///
/// Instructions:
///     - Use this tool multiple times in order achieve complex tasks
///     - Before execute one or more commands, output them in the answer and wait the user to
///     confirm
///     - After you launched on or more commands, write the output in the answer and explain what
///     you did
#[ollama_rs::function]
pub async fn exec_command(command: String) -> Result<String, anyhow::Error> {
    match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => {
            let string = String::from_utf8(output.stdout).expect("Invalid UTF-8 stdout");
            Ok(string)
        }
        Err(e) => Err(Error::ToolCalling {
            tool_name: "exec_command".to_string(),
            error: e.to_string(),
        }
        .into()),
    }
}

/// Open and read an existent file
///
/// Args:
///     - path: path of the file to be read
#[ollama_rs::function]
pub async fn read_file(path: String) -> Result<String, anyhow::Error> {
    match File::open(path) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(Error::ToolCalling {
                    tool_name: "read_file".to_string(),
                    error: e.to_string(),
                }
                .into()),
            }
        }
        Err(e) => Err(Error::ToolCalling {
            tool_name: "read_file (open it)".to_string(),
            error: e.to_string(),
        }
        .into()),
    }
}

/// Make a new file
///
/// Args:
///     -path: path of the file to be created
#[ollama_rs::function]
pub async fn create_file(path: String) -> Result<String, anyhow::Error> {
    match File::create(path) {
        Ok(_) => Ok("File creato!".to_string()),
        Err(e) => Err(Error::ToolCalling {
            tool_name: "create_file".to_string(),
            error: e.to_string(),
        }
        .into()),
    }
}
