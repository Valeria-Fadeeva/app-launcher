extern crate dotenv;

use dotenv::dotenv;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::{env, str};

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let python_path = env::var("PYTHON_PATH").unwrap();
    let python_path = python_path.as_str();

    let app: &str = if cfg!(target_os = "windows") {
        python_path
    } else if cfg!(target_os = "linux") {
        "python3"
    } else {
        "python3"
    };

    let par = "-s";

    let execute = env::var("EXECUTE").unwrap();
    let execute = execute.as_str();

    let child = if args.len() == 1 {
        Command::new(app)
            .args([execute])
            .stderr(Stdio::null()) // don't care about stderr
            .stdout(Stdio::inherit()) // set up stdout so we can read it
            .stdin(Stdio::inherit()) // set up stdin so we can write on it
            .spawn()
            .expect("Could not run the command") // finally run the command
    } else {
        Command::new(app)
            .args([execute, par, &args[1]])
            .stderr(Stdio::null()) // don't care about stderr
            .stdout(Stdio::inherit()) // set up stdout so we can read it
            .stdin(Stdio::inherit()) // set up stdin so we can write on it
            .spawn()
            .expect("Could not run the command") // finally run the command
    };

    let output = child.wait_with_output().expect("Failed to wait on child");

    let stdout = output.stdout.as_slice();
    let out = str::from_utf8(&stdout).unwrap();

    println!("{}", out);

    io::stdout().flush().unwrap();
}
