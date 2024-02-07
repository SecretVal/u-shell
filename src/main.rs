use std::io::{stdin, stdout, BufRead, Write};

struct Command {
    name: String,
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
            action: || {
                // TODO: Look into this
                let _ = stdout().write(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes());
                let _ = stdout().flush();
            },
        },
        Command {
            name: "ping".to_string(),
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
        }
    }
}
