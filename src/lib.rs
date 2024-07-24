use inquire::{required, Text};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub email: String,
    pub name: String,
    pub alias: String,
}

pub static FILE_PATH: &str = "./config.json";

pub fn set_text(message: &str, default: Option<&str>) -> String {
    let mut text = Text::new(message).with_validator(required!("This field is required"));

    text = match default {
        Some(value) => text.with_default(value),
        None => text,
    };

    text.prompt().unwrap()
}

pub fn read_config() -> Option<String> {
    // 检查文件是否存在
    if Path::new(FILE_PATH).exists() {
        // 如果文件存在，读取文件内容
        let mut file = File::open(FILE_PATH).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        Some(content)
    } else {
        None
    }
}

pub fn set_git_config(config: &Config) {
    let command = "git";
    exec_command(command, ["config", "user.name", config.name.as_str()]);
    exec_command(command, ["config", "user.email", config.email.as_str()]);
    println!("\n✅ config success");
}

fn exec_command(command: &str, args: [&str; 3]) {
    let _ = Command::new(command)
        .args(args)
        .output()
        .expect("❌ Failed to execute command");
}

pub fn handle_write(file_write: io::Result<()>) {
    match file_write {
        Ok(_) => println!("\n✅ add success"),
        Err(_) => println!("\n❌ add failed"),
    }
}
