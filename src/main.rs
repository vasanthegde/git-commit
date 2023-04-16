use std::{
    env,
    process::{Command, Output},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];

    let output: Output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(args.first().expect("commit message required"))
        .output()
        .expect("Failed to run git commit command");

    let dirty_out = String::from_utf8_lossy(&output.stdout);
    println!("{}", dirty_out);
}
