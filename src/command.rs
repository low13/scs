use std::io::{self, Write};
use std::{process, env, fs};
use std::path::Path;

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