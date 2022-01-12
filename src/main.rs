use std::process::{Command, Stdio};
use std::{env, fs, str};

fn read_file(watch_path: &str) -> String {
    let filepath: String;

    let filepath_r = fs::read_to_string(watch_path);
    filepath = match filepath_r {
        Ok(filepath) => filepath,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    return filepath;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let python_path = ".env";
    let p = read_file(python_path);

    let app: &str = if cfg!(target_os = "windows") {
        &*p.trim()
    } else if cfg!(target_os = "linux") {
        "python3"
    } else {
        "python3"
    };

    let execute = "mock-command.py";

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
            .args([execute, &args[1]])
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
}
