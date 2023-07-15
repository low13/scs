use std::fs;
use std::process::Command;

fn main() {
    let _ = Command::new("cargo").args(["build", "--release"]).status();
    let _ = fs::copy("target/release/scs.exe", "out/scs.exe");

    let _ = fs::create_dir("out");
    let _ = fs::create_dir("out/commands");
    for command in fs::read_dir("commands").unwrap() {
        let command_path = command.unwrap().path();
        let _ = Command::new("cargo")
            .args([
                "build",
                "--release",
                "--manifest-path",
                command_path.join("Cargo.toml").to_str().unwrap()])
            .status();
        let _ = fs::copy(
            command_path.join("target/release")
                .join(command_path.file_name().unwrap()).to_str().unwrap().to_owned() + ".exe",
            format!("out/commands/{}.exe", command_path.file_name().unwrap().to_string_lossy()));
    }
}