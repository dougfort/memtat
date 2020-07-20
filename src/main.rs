use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    prompt()?;
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        let command: Vec<&str> = line.trim().split_whitespace().map(|s| s.trim()).collect();

        if command.is_empty() {
            prompt()?;
            continue;
        }
        match command[0] {
            "" => (),
            "quit" => {
                println!();
                break;
            },
            _ => {
                println!("invalid input: {}", command[0]);
            }
        }

        prompt()?;
    }

    Ok(())
}

fn prompt() -> io::Result<()> {
    println!();
    print!("> ");
    io::stdout().flush()?;

    Ok(())
}
