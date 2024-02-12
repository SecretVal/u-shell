mod math;
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, Write},
};

#[derive(Debug, Clone)]
struct Command {
    name: String,
    desc: String,
    action: fn(
        args: Vec<&str>,
        commmands: HashMap<String, Command>,
        prompt: String,
    ) -> (bool, HashMap<String, Command>, String),
}
fn main() {
    let mut commands: HashMap<String, Command> = HashMap::new();
    commands.insert(
        "clear".to_string(),
        Command {
            name: "clear".to_string(),
            desc: "Clear the screen".to_string(),
            action: |_, commands: HashMap<String, Command>, prompt: String| {
                let _ = stdout().write(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes());
                let _ = stdout().flush();
                return (false, commands, prompt);
            },
        },
    );
    commands.insert(
        "ping".to_string(),
        Command {
            name: "ping".to_string(),
            desc: "Say pong".to_string(),
            action: |_, commands: HashMap<String, Command>, prompt: String| {
                let _ = stdout().write(b"pong\n");
                let _ = stdout().flush();
                return (false, commands, prompt);
            },
        },
    );
    commands.insert(
        "help".to_string(),
        Command {
            name: "help".to_string(),
            desc: "Print help message ".to_string(),
            action: |_, commands: HashMap<String, Command>, prompt: String| {
                for cmd in commands.values() {
                    let _ = stdout().write(format!("{}: {}\n", cmd.name, cmd.desc).as_bytes());
                    let _ = stdout().flush();
                }
                return (false, commands, prompt);
            },
        },
    );
    commands.insert(
        "exit".to_string(),
        Command {
            name: "exit".to_string(),
            desc: "Quit the shell".to_string(),
            action: |_, commands: HashMap<String, Command>, prompt: String| {
                return (false, commands, prompt);
            },
        },
    );
    commands.insert(
        "math".to_string(),
        Command {
            name: "math".to_string(),
            desc: "math".to_string(),
            action: |_, mut commands: HashMap<String, Command>, mut prompt: String| {
                if let Some(_) = commands.get("add") {
                    commands = math::remove(commands.clone());
                    prompt = prompt.replace("(math)", "");
                } else {
                    commands = math::add(commands.clone());
                    prompt = String::from("(math)") + &prompt;
                };
                return (false, commands, prompt);
            },
        },
    );
    let mut exit = false;
    let mut prompt: String = String::from("->");
    while !exit {
        let mut handler = stdin().lock();
        let _ = stdout().write(prompt.as_bytes());
        let _ = stdout().flush();
        let mut buffer = String::new();
        let _ = handler.read_line(&mut buffer);
        let command = &commands.get(buffer.trim());

        let mut args = buffer.split(" ").into_iter().collect::<Vec<&str>>();
        args.remove(0);
        (exit, commands, prompt) = (command
            .unwrap_or(&Command {
                name: "NOT_FOUND".to_string(),
                desc: "NOT_FOUND".to_string(),
                action: |_, commands: HashMap<String, Command>, prompt: String| {
                    let _ = stdout().write(b"Command not found.\n");
                    let _ = stdout().flush();
                    return (false, commands, prompt);
                },
            })
            .action)(args, commands.clone(), prompt);
    }
}
