use std::env;
use std::io;
use std::io::Write;
use std::process::Command;

use clap::{App, Arg};
use colored::Colorize;
use log::LevelFilter;
use rustyline::Editor;
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};

use prompt2shell::openai::{create_client, generate_command};
use prompt2shell::prompt::PROMPT;
use prompt2shell::utils::{log, warning};

#[tokio::main]
async fn go(
    request: &mut String,
    openai_api_key: &str,
    unsafe_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut prompt = PROMPT.to_string();

    // Add prompt from stdin (arg1 from the function)
    prompt.push_str(request);

    // Start OpenAI client
    let client = create_client(openai_api_key.to_string()).await;

    let mut command = match generate_command(&client, &prompt.to_string()).await {
        Ok(command) => command.to_string(),
        Err(e) => e.to_string(),
    };
    println!("{} {}", "$", command.green());

    log(&("[Command] ".to_owned() + &command.to_string()));

    let mut execute_command = false;
    let mut edited_command = command.clone();

    if !unsafe_mode {
        let mut rl = Editor::<()>::new();

        // Déplacez cette ligne à l'intérieur de la boucle if
        loop {
            let mut run_it = String::new();
            print!(
                "Run it ({}es/{}o/{}dit): ",
                "Y".yellow(),
                "N".yellow(),
                "E".yellow()
            );
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut run_it).expect("error");
            let run_it = run_it.trim().to_lowercase();
            if run_it == "y" {
                execute_command = true;
                break;
            } else if run_it == "n" {
                break;
            } else if run_it == "e" {
                match rl.readline_with_initial("$ ", (&command, "")) {
                    Ok(line) => {
                        edited_command = line.trim().to_string();
                        command = edited_command.clone();
                        //println!("{} {}", "$", final_command.green());
                    }
                    Err(_) => {
                        println!("Error reading modified command.");
                    }
                }
            } else {
                println!("Invalid input. Please enter 'y', 'n', or 'modify'.");
            }
        }
    }
    if execute_command {
        //Command::new(&command).spawn()?;
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Failted to execute command.")
            .wait_with_output()
            .expect("Failted to wait on child process.");

        // Collect stdout
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Print stdout
        if output.status.success() {
            println!("{}", output_str);
        } else {
            println!("Erreur :{}", String::from_utf8_lossy(&output.stderr));
        }

        // To delete ?
    }
    Ok(())
}

fn main() {
    // Vérification de la présence de la variable d'environnement OPENAI_API_KEY
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Erreur : La variable d'environnement OPENAI_API_KEY n'est pas définie.");
            std::process::exit(1);
        }
    };

    let log_config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S%.3f") // Format de la date et de l'heure
        .build();

    let log_file = std::fs::File::create(".prompt2shell.log").unwrap();

    CombinedLogger::init(vec![
        //TermLogger::new(LevelFilter::Info, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, log_config, log_file),
    ])
    .unwrap();

    //SimpleLogger::init(LevelFilter::Info, log_config).unwrap();

    let matches = App::new("prompt2shell")
        .version("0.1")
        .arg(Arg::new("unsafe").short('u').long("unsafe"))
        .get_matches();

    let unsafe_mode = matches.is_present("unsafe");
    warning(unsafe_mode);

    // Loop to create a CLI
    loop {
        print!("[{}]> ", "prompt2shell".yellow());
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).expect("error");
        let my_str = prompt.trim().to_string();
        log(&("[Prompt] ".to_owned() + &my_str));
        let prompt2 = prompt.trim();
        if prompt2 == "quit" {
            break;
        } else {
            match go(&mut prompt, &openai_api_key, unsafe_mode) {
                Err(_) => (),
                _ => (),
            };
        }
    }
}
