use std::io::{self, Write};
use std::{process, env, fs};
use std::path::Path;
use std::collections::HashMap;

pub fn help() {
    println!("\nCOMMANDS");
    let mut commands_help: HashMap<&str, &str> = HashMap::new();

    commands_help.insert("echo", "Prints the given text to the output");
    commands_help.insert("cd", "Changes the current working directory to the specified path");
    commands_help.insert("clear", "Clears all text in the window");
    commands_help.insert("ls", "Lists all items in the current working directory");
    commands_help.insert("mkfile", "Creates the given file if it doesn't exist yet");
    commands_help.insert("rmfile", "Permanently removes the given file from the file system");
    commands_help.insert("mkdir", "Creates the given directory if it doesn't exist yet");
    commands_help.insert("rmdir", "Permanently removes the given directory if empty, using '--force' also removes non-empty directories");
    commands_help.insert("read", "Reads the text contained in a file with UTF-8 encoding");
    commands_help.insert("copy", "Copies content from path A to path B");
    commands_help.insert("move", "Moves content from path A to path B, can be used to rename items");
    commands_help.insert("find", "Prints the path for the given executable if found on the PATH variable");
    commands_help.insert("run", "Attempts to run the given file with the given arguments, can be used to execute external commands");
    commands_help.insert("help", "Lists all commands");
    commands_help.insert("exit", "Exit the shell");

    for (cmd, desc) in commands_help {
        println!("{}: {}", cmd, desc);
    }
}

pub fn exit() {
    process::exit(0);
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;0H");
    io::stdout().flush().unwrap();
}

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

pub fn cd(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if let Err(_) = env::set_current_dir(path) {
        println!("Unable to access directory");
    }
}

pub fn ls() {
    if let Ok(entries) = fs::read_dir(env::current_dir().unwrap()) {
        for entry in entries {
            if let Ok(entry) = entry {
                print!("   {}", entry.file_name().to_string_lossy());
            }
        }
        println!();
    } else {
        println!("Unable to read directory");
    }
}

pub fn mkfile(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::File::create(&args);
    }
}

pub fn rmfile(args: Vec<&str>) {
    if args.len() == 0 {
        println!("Expected file");
        return;
    }

    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() && path.is_file() {
        let _ = fs::remove_file(&path);
    } else {
        println!("Unable to find file '{}'", &args);
    }
}

pub fn mkdir(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);
    if path.exists() {
        println!("'{}' already exists", &args);
    } else {
        let _ = fs::create_dir_all(&args);
    }
}

pub fn rmdir(args: Vec<&str>) {
    let args_temp;
    let mut force = false;

    if args.len() == 0 {
        println!("Expected directory");
        return;
    } else if args.len() == 1 && args[0] == "--force" {
        println!("Expected directory");
        return;
    }

    if args[0] == "--force" {
        force = true;
        args_temp = args[1..].join(" ");
    } else {
        args_temp = args.join(" ");
    }

    let args = args_temp;

    let path = Path::new(&args);
    if path.exists() && path.is_dir() {
        if let Ok(entries) = fs::read_dir(&path) {
            if entries.count() == 0 {
                let _ = fs::remove_dir(&path);
            } else if force {
                let _ = fs::remove_dir_all(&path);
            } else {
                println!("Directory '{}' is not empty. Consider using '--force' to delete it anyway", args);
            }
        }
    } else {
        println!("Unable to find directory '{}'", &args);
    }
}

pub fn read(args: Vec<&str>) {
    let args = args.join(" ");
    let path = Path::new(&args);

    if path.exists() && path.is_file() {
        println!("{}", fs::read_to_string(path).unwrap());
    } else {
        println!("Unable to read file '{}'", &args);
    }
}

pub fn copy(args: Vec<&str>) {
    if args.len() == 0 {
        println!("Expected source path");
        return;
    }
    if args.len() == 1 {
        println!("Expected destination path");
        return;
    }

    let src_path = Path::new(args[0]);
    let dest_path = Path::new(args[1]);

    if !src_path.exists() {
        println!("Unable to find '{}'", src_path.display());
        return;
    }

    let _ = fs::copy(src_path, dest_path);
}

pub fn mv(args: Vec<&str>) {
    if args.len() == 0 {
        println!("Expected source path");
        return;
    }
    if args.len() == 1 {
        println!("Expected destination path");
        return;
    }

    let src_path = Path::new(args[0]);
    let dest_path = Path::new(args[1]);

    if !src_path.exists() {
        println!("Unable to find '{}'", src_path.display());
        return;
    }

    let _ = fs::rename(src_path, dest_path);
}

pub fn find(args: Vec<&str>) {
    if args.len() == 0 {
        println!("Expected path");
        return;
    }

    if let Ok(path_var) = env::var("PATH") {
        let mut file_name = args[0..].join(" ");
        if !file_name.ends_with(".exe") {
            file_name += ".exe";
        }
        let mut finished = false;

        for path in env::split_paths(&path_var) {
            let candidate = path.join(file_name.clone());
            if candidate.is_file() {
                finished = true;
                println!("{}", candidate.display());
                break;
            }
        }
        if finished == false {
            println!("Unable to find file {}", args[0..].join(" "));
        }
    }
}

pub fn run(args: Vec<&str>) {
    if args.len() == 0 {
        println!("Expected file");
        return;
    }

    let result = process::Command::new(args[0]).args(&args[1..]).status();

    if let Ok(status) = result {
        if status.success() {
            return;
        }
    }
}