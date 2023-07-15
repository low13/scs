use std::fs;
use std::env::args;
use std::path::Path;

fn main() {
    let args = &args().collect::<Vec<String>>()[1..];
    let args_temp;
    let mut force = false;

    if args.len() == 0 {
        println!("Usage: rmdir {} {}", "{-f | --force}?", "{directory}");
        return;
    } else if args.len() == 1 && args[0] == "--force" {
        println!("Usage: rmdir {} {}", "{-f | --force}?", "{directory}");
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