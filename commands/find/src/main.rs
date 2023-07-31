use std::env;

fn main() {
    let args = &env::args().collect::<Vec<String>>()[1..];

    if args.len() == 0 {
        println!("Usage: find {}", "{file}");
        return;
    }

    if let Ok(path_var) = env::var("PATH") {
        let mut file_name = args.join(" ");
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
            println!("Unable to find file {} on PATH", args[0..].join(" "));
        }
    }
}