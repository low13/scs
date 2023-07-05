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
        let args = input.split_whitespace().skip(1).collect::<Vec<&str>>();

        match cmd {
            "exit" => exit(0),
            "echo" => println!("{}", args.join(" ")),
            _ => println!("Command not found: {}", cmd)
        }
    }
}