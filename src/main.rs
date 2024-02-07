use std::io::{stdin, stdout, BufRead, Write};

#[derive(Debug)]
struct Command {
    name: String,
    desc: String,
    action: fn(),
}
impl Command {
    fn run(&self) {
        (self.action)();
    }
}
fn main() -> ! {
    let commands: Vec<Command> = vec![
        Command {
            name: "clear".to_string(),
            desc: "Clear the screen".to_string(),
            action: || {
                // TODO: Look into this
                let _ = stdout().write(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes());
                let _ = stdout().flush();
            },
        },
        Command {
            name: "ping".to_string(),
            desc: "say pong".to_string(),
            action: || {
                let _ = stdout().write(b"pong\n");
                let _ = stdout().flush();
            },
        },
    ];
    loop {
        let mut handler = stdin().lock();
        let _ = stdout().write(b"-> ");
        let _ = stdout().flush();
        let mut buffer = String::new();
        let _ = handler.read_line(&mut buffer);
        for cmd in &commands {
            if cmd.name == buffer.trim() {
                cmd.run();
            }
            if buffer.trim() == "help".to_string() {
                let _ = stdout().write(format!("{}: {}\n", cmd.name, cmd.desc).as_bytes());
                let _ = stdout().flush();
            }
        }
    }
}
