use clap::{arg, command};

fn main() {
    let matches = command!()
        .subcommand(
            command!("set")
                .arg(arg!(--key <VALUE>))
                .arg(arg!(--value <VALUE>)),
        )
        .subcommand(command!("list"))
        .subcommand(command!("add"))
        .subcommand(command!("switch"))
        .get_matches();

    match matches.subcommand() {
        Some(("set", _)) => handle_set(),
        Some(("switch", _)) => handle_switch(),
        Some(("list", _)) => handle_list(),
        Some(("add", _)) => handle_add(),
        _ => println!("No valid subcommand was used. You can try -h or --help for know more"),
    }
}
fn handle_set() {
    println!("Setting {} to {}", 1, 1);
    // 实现你的逻辑
}

fn handle_list() {
    println!("Listing items");
    // 实现你的逻辑
}

fn handle_add() {
    println!("Adding ");
    // 实现你的逻辑
}

fn handle_switch() {
    println!("Switching to {}", 1);
    // 实现你的逻辑
}
