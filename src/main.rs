use std::io::{self, Write};
use std::process::exit;

fn main() {
    println!("SCS System Command Shell 1.0.0");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.split_whitespace().collect::<Vec<&str>>()[0];

        match cmd {
            "exit" => exit(0),
            _ => println!("Command not found: {}", cmd)
        }
    }
}