use std::env;
use std::io::{self, Write};
mod command;

fn main() {
    println!("SCS System Command Shell 1.0.0");
    println!("Enter 'help' for a list of all commands");

    loop {
        print!("{} => ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let mut invalid_input = false;
        let _ = io::stdin().read_line(&mut input).map_err(|_| {
            eprintln!("\nInvalid input found");
            invalid_input = true;
        });

        if invalid_input { continue };

        let cmd = {
            let cmd_opt = input.split_whitespace().next();
            if cmd_opt.is_none() {
                continue;
            }
            cmd_opt.unwrap().to_owned()
        };
        

        let args = input.split_whitespace().skip(1).collect::<Vec<&str>>();

        match cmd.as_str() {
            "help" => command::help(),
            "exit" => command::exit(),
            "clear" => command::clear(),
            "echo" => command::echo(args),
            "cd" => command::cd(args),
            "ls" => command::ls(),
            "mkfile" => command::mkfile(args),
            "rmfile" => command::rmfile(args),
            "mkdir" => command::mkdir(args),
            "rmdir" => command::rmdir(args),
            "read" => command::read(args),
            "copy" => command::copy(args),
            "move" => command::mv(args),
            "find" => command::find(args),
            "run" => command::run(args),
            _ => println!("Command not found: {}", cmd)
        }
    }
}