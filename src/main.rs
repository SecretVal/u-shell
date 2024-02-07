use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, Write},
};

#[derive(Debug)]
struct Command {
    desc: String,
    action: fn(args: Vec<String>),
}
fn main() -> ! {
    let mut commands: HashMap<String, Command> = HashMap::new();
    commands.insert(
        "clear".to_string(),
        Command {
            desc: "Clear the screen".to_string(),
            action: |_| {
                // TODO: Look into this
                let _ = stdout().write(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes());
                let _ = stdout().flush();
            },
        },
    );
    commands.insert(
        "ping".to_string(),
        Command {
            desc: "Say pong".to_string(),
            action: |_| {
                let _ = stdout().write(b"pong\n");
                let _ = stdout().flush();
            },
        },
    );
    commands.insert(
        "help".to_string(),
        Command {
            desc: "Print help".to_string(),
            action: |_| {
                for cmd in commands.values() {
                    println!("{:?}", cmd);
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
        let command = commands.get(buffer.trim());
        (command
            .unwrap_or(&Command {
                desc: "NOT_FOUND".to_string(),
                action: |_| {
                    let _ = stdout().write(b"Command not found.\n");
                    let _ = stdout().flush();
                },
            })
            // TODO: implement args
            .action)(vec![]);
    }
}
