use std::env;
use std::process;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    println!("SCS System Command Shell 1.0.0");

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

        if cmd == "cd" {
            let args = args.join(" ");
            let path = Path::new(&args);
            if let Err(_) = env::set_current_dir(path) {
                println!("Unable to access directory");
            }
        }
        if cmd == "exit" {
            process::exit(0);
        }
        
        if Path::new(&cmd).exists() {
            let _ = process::Command::new(&cmd).args(args).status();
        } else {
            let cmd_exe = cmd.to_string() + ".exe";
            if Path::new(&cmd_exe).exists() {
                let _ = process::Command::new(&cmd_exe).args(args).status();
            } else {
                println!("Command not found: {}", cmd);
            }
        }
    }
}