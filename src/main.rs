// Modules declaration
mod cli;
mod errors;
mod tools;

// Necessary imports
use cli::Args;
use colored::Colorize;
use errors::*;
use figlet_rs::FIGfont;
use ollama_rs::{
    coordinator::Coordinator,
    generation::{
        chat::ChatMessage,
        tools::implementations::{Browserless, DDGSearcher, Scraper},
    },
    Ollama,
};
use regex::Regex;
use std::{
    fs::File,
    io::{stdin, Read},
};
use structopt::StructOpt;
use tokio::io::{stdout, AsyncWriteExt};
use tools::*;

// Main program
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Reads arguments from cli
    let Args { model, prompt } = Args::from_args();
    if model.is_empty() {
        return Err(CliError::ModelError.into());
    }

    title().await?; // Clears the screen and displays the title

    // Variables setting
    let ollama = Ollama::default();
    let history = vec![];
    let mut stdout = stdout();

    let default_prompt = "Sei un assistente AI nel terminale, il tuo nome è R2D2 ma puoi essere soprannominato solo R2.
ISTRUZIONI:
Hai a disposizione vari tools per compiere le azioni richieste.
Sei un agente da terminale, pertanto ti troverai spesso ad eseguire comandi sulla shell, interagire col file system e col sistema in generale.
Prima di eseguire un comando mostralo sempre nella risposta chiedendo conferma.
Dai sempre una risposta dopo aver eseguito qualsiasi tool, spiegando cosa hai fatto. in che modo e che tool hai utilizzato con quali argomenti.
Parla in italiano sempre.";

    let mut system_prompt: String = default_prompt.to_string(); // System prompt is set to default

    // Checks is user specified a custom prompt and sets is as the system prompt
    if prompt.is_some() {
        match File::open(prompt.unwrap()) {
            Ok(mut f) => {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => system_prompt = contents.clone(),
                    Err(_) => return Err(CliError::PromptError {
                        error:
                            "Failed to read the content of the specified file, make sure it is txt"
                                .to_string(),
                    }
                    .into()),
                }
            }
            Err(_) => {
                return Err(CliError::PromptError {
                    error: "Failed to open the specified file, make sure it exists".to_string(),
                }
                .into())
            }
        }
    }

    // Ollama-rs setting
    let system_message = ChatMessage::system(system_prompt);

    let mut coordinator = Coordinator::new(ollama, model.to_string(), history)
        .add_tool(DDGSearcher::new())
        .add_tool(Scraper {})
        .add_tool(Browserless {})
        .add_tool(exec_command)
        .add_tool(read_file)
        .add_tool(create_file);

    let re = Regex::new(r"(?s)<think>.*?</think>\n?").unwrap(); // Hides thinking in the response

    // Main loop of the program
    loop {
        stdout.write_all(b"\n\n> ").await?;
        stdout.flush().await?;

        // Input from the user
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim_end();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let user_message = ChatMessage::user(input.to_string());

        // Response from the model
        let resp = coordinator
            .chat(vec![system_message.clone(), user_message])
            .await?;

        let output = resp.message.content;

        let fixed_output = re.replace_all(&output, "").into_owned();

        stdout.write_all(fixed_output.as_bytes()).await?;
        stdout.flush().await?;
    }

    Ok(())
}

// Clears the screen and displays the title
async fn title() -> Result<(), anyhow::Error> {
    if cfg!(target_os = "windows") {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    match FIGfont::standard() {
        Ok(std_font) => {
            let text = std_font.convert("R2D2").unwrap();
            println!("{}", text.to_string().blue().bold());
            Ok(())
        }
        Err(_) => Err(Error::Figlet.into()),
    }
}
