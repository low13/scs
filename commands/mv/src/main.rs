use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>()[1..];

    if args.len() < 2 {
        println!("Usage move {} {}", "{source}", "{destination}");
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