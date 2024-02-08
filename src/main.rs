use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, Write},
};

#[derive(Debug, Clone)]
struct Command {
    name: String,
    desc: String,
    action: fn(args: Vec<String>, commmands: HashMap<String, Command>) -> bool,
}
fn main() {
    let mut commands: HashMap<String, Command> = HashMap::new();
    commands.insert(
        "clear".to_string(),
        Command {
            name: "clear".to_string(),
            desc: "Clear the screen".to_string(),
            action: |_, _| {
                // TODO: Look into this
                let _ = stdout().write(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes());
                let _ = stdout().flush();
                return false;
            },
        },
    );
    commands.insert(
        "ping".to_string(),
        Command {
            name: "ping".to_string(),
            desc: "Say pong".to_string(),
            action: |_, _| {
                let _ = stdout().write(b"pong\n");
                let _ = stdout().flush();
                return false;
            },
        },
    );
    commands.insert(
        "help".to_string(),
        Command {
            name: "help".to_string(),
            desc: "Print help message ".to_string(),
            action: |_, commands: HashMap<String, Command>| {
                for cmd in commands.values() {
                    let _ = stdout().write(format!("{}: {}\n", cmd.name, cmd.desc).as_bytes());
                    let _ = stdout().flush();
                }
                return false;
            },
        },
    );
    commands.insert(
        "exit".to_string(),
        Command {
            name: "exit".to_string(),
            desc: "Quit the shell".to_string(),
            action: |_, _| {
                return true;
            },
        },
    );
    let mut exit = false;
    while !exit {
        let mut handler = stdin().lock();
        let _ = stdout().write(b"->");
        let _ = stdout().flush();
        let mut buffer = String::new();
        let _ = handler.read_line(&mut buffer);
        let command = &commands.get(buffer.trim());
        exit = (command
            .unwrap_or(&Command {
                name: "NOT_FOUND".to_string(),
                desc: "NOT_FOUND".to_string(),
                action: |_, _| {
                    let _ = stdout().write(b"Command not found.\n");
                    let _ = stdout().flush();
                    return false;
                },
            })
            // TODO: implement args
            .action)(vec![], commands.clone());
    }
}
