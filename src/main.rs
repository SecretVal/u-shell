use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, Write},
};

#[derive(Debug, Clone)]
struct Command {
    name: String,
    desc: String,
    action: fn(args: Vec<String>, commmands: HashMap<String, Command>),
}
fn main() -> ! {
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
            },
        },
    );
    loop {
        let mut handler = stdin().lock();
        let _ = stdout().write(b"->");
        let _ = stdout().flush();
        let mut buffer = String::new();
        let _ = handler.read_line(&mut buffer);
        let command = &commands.get(buffer.trim());
        (command
            .unwrap_or(&Command {
                name: "NOT_FOUND".to_string(),
                desc: "NOT_FOUND".to_string(),
                action: |_, _| {
                    let _ = stdout().write(b"Command not found.\n");
                    let _ = stdout().flush();
                },
            })
            // TODO: implement args
            .action)(vec![], commands.clone());
    }
}
