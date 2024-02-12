use crate::Command;
use std::{
    collections::HashMap,
    io::{stdout, Write},
};

pub fn add(mut commands: HashMap<String, Command>) -> HashMap<String, Command> {
    commands.insert(
        "add".to_string(),
        Command {
            name: "add".to_string(),
            desc: "add 1 2".to_string(),
            action: |args: Vec<&str>, commands: HashMap<String, Command>, prompt: String| {
                if args.len() == 2 {
                    let _ = stdout().write(
                        format!(
                            "{}\n",
                            args[0].trim().parse::<i32>().unwrap()
                                + args[1].trim().parse::<i32>().unwrap()
                        )
                        .as_bytes(),
                    );
                    let _ = stdout().flush();
                    return (false, commands, prompt);
                } else {
                    return (false, commands, prompt);
                }
            },
        },
    );
    return commands;
}

pub fn remove(mut commands: HashMap<String, Command>) -> HashMap<String, Command> {
    commands.remove("add");
    return commands;
}
