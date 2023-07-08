use std::env;
use std::io::{self, Write};
mod command;

fn main() {
    println!("SCS System Command Shell 0.2.0");
    loop {
        print!("{}> ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = {
            let cmd_opt = input.split_whitespace().next();
            if cmd_opt.is_none() {
                continue;
            }
            cmd_opt.unwrap().to_owned()
        };
        

        let args = input.split_whitespace().skip(1).collect::<Vec<&str>>();

        match cmd.as_str() {
            "exit" => command::exit(),
            "echo" => command::echo(args),
            "cd" => command::cd(args),
            "ls" => command::ls(),
            "mkfile" => command::mkfile(args),
            "rmfile" => command::rmfile(args),
            "mkdir" => command::mkdir(args),
            "rmdir" => command::rmdir(args),
            _ => println!("Command not found: {}", cmd)
        }
    }
}