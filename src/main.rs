use std::{fs::File, io::Write};

use clap::command;
use gitu::{handle_write, read_config, set_git_config, set_text, Config, FILE_PATH};
use inquire::{InquireError, Select};

fn main() {
    let matches = command!()
        .subcommand(command!("add"))
        .subcommand(command!("switch"))
        .get_matches();

    match matches.subcommand() {
        Some(("switch", _)) => handle_switch(),
        Some(("add", _)) => handle_add(),
        _ => println!("No valid subcommand was used. You can try -h or --help for know more"),
    }
}

fn handle_add() {
    let email = set_text("set your git email", None);
    let name = set_text("set your git name", None);
    let alias = set_text("set your git alias", Some(&name));

    let config = Config { name, email, alias };

    match read_config() {
        Some(content) => {
            let mut content: Vec<Config> = serde_json::from_str(&content).unwrap();
            content.push(config);

            let mut file = File::create(FILE_PATH).unwrap();
            let content = serde_json::to_string(&content).unwrap();

            handle_write(file.write_all(content.as_bytes()));
        }

        None => {
            let mut file = File::create(FILE_PATH).unwrap();
            let content = serde_json::to_string(&vec![config]).unwrap();

            handle_write(file.write_all(content.as_bytes()));
        }
    }
}

fn handle_switch() {
    match read_config() {
        Some(content) => {
            let configs: Vec<Config> = serde_json::from_str(&content).unwrap();

            let options = configs
                .iter()
                .enumerate()
                .map(|(index, config)| {
                    format!(
                        "{}) alias: {}, name: {}, email: {}",
                        index + 1,
                        config.alias,
                        config.name,
                        config.email
                    )
                })
                .collect();

            let ans: Result<String, InquireError> =
                Select::new("What's your favorite fruit?", options).prompt();

            match ans {
                Ok(choice) => {
                    let index = choice.chars().next().unwrap().to_digit(10).unwrap() as usize;
                    set_git_config(&configs[index - 1]);
                }
                Err(_) => println!("\n ❌ cancel to switch"),
            }
        }
        None => {
            println!("No config, you can run `gitu add` to add a new config");
        }
    }
}
