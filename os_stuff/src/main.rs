use std::process::Command;

fn main() {
    // 1. Get Rust version
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute command! Command not found or invalid format");
    if output.status.success() {
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    // 2. operating system details
    // First method, using cfg! macro
    if cfg!(target_os = "linux") {
        println!("On linux");
    }
    // Second method, using env variables, consts
    println!("OS: {}", std::env::consts::OS);
    // 3. system architecture
    // Here, we can use the same method, just with ARCH
    println!("ARCH: {}", std::env::consts::ARCH);
}
