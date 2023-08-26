use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    // cargo run -- test=hola

    let gitstatus = Command::new("git")
        .arg("-C")
        .arg("./test-brick")
        .arg("status")
        .output()
        .expect("ls command failed to start");

    if gitstatus.status.success() {
        let stdout = String::from_utf8_lossy(&gitstatus.stdout);
        let untracked_files = extract_untracked_files(&stdout);

        for file in untracked_files {
            println!("{}", file);
        }
    } else {
        eprintln!("Error running git command:\n{}", String::from_utf8_lossy(&gitstatus.stderr));
    }
}

fn extract_untracked_files(output: &str) -> Vec<&str> {
    let mut untracked_files = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    let mut in_untracked_section = false;

    for line in lines {
        if line.starts_with("Untracked files:") {
            in_untracked_section = true;
            continue;
        } else if in_untracked_section && line.is_empty() {
            in_untracked_section = false;
            continue;
        }
        
        if in_untracked_section && line.starts_with('\t') {
            untracked_files.push(line.trim_start_matches('\t'));
        }
    }

    untracked_files
}
