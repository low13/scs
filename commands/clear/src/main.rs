use std::io::{self, Write};

fn main() {
    print!("\x1B[2J\x1B[1;0H");
    io::stdout().flush().unwrap();
}