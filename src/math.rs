use crate::Command;
use std::collections::HashMap;

pub fn add(mut commands: HashMap<String, Command>) -> HashMap<String, Command> {
    commands.insert(
        "add".to_string(),
        Command {
            name: "add".to_string(),
            desc: "add 1 2".to_string(),
            action: |_, commands: HashMap<String, Command>, prompt: String| {
                println!("3");
                return (false, commands, prompt);
            },
        },
    );
    return commands;
}

pub fn remove(mut commands: HashMap<String, Command>) -> HashMap<String, Command> {
    commands.remove("add");
    return commands;
}
