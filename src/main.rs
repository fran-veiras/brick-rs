use std::process::Command;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config_file {
    pm: String,
    jobs: Vec<String>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // cargo run -- test=hola

    let gitstatus = Command::new("git")
        .arg("-C")
        .arg("./")
        .arg("status")
        .output()
        .expect("ls command failed to start");

    if gitstatus.status.success() {
        ci_brick(gitstatus)
    } else {
        eprintln!("Error running git command:\n{}", String::from_utf8_lossy(&gitstatus.stderr));
        Ok(())
    }
}

fn ci_brick (gitstatus : std::process::Output) -> Result<()> {
    let stdout = String::from_utf8_lossy(&gitstatus.stdout);
    let untracked_files = extract_untracked_files(&stdout);
    let not_staged_files = extract_not_staged(&stdout);
    
    let contents = fs::read_to_string("./brick.config.json")
        .expect("Should have been able to read the file");
    let p:Config_file = serde_json::from_str(&contents)?;

    println!("hello, {:?}", p.pm);

    for file in untracked_files {
        // println!("Untracked {}", file);
    }


    for file in not_staged_files {
        // println!("Not staged {}", file);
    }

    Ok(())
}

fn extract_not_staged(output: &str) -> Vec<&str> {
    let mut not_staged_files = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    let mut in_not_staged_section = false;

    for line in lines {
        if line.starts_with("Changes not staged for commit:") {
            in_not_staged_section = true;
            continue;
        } else if in_not_staged_section && line.is_empty() {
            in_not_staged_section = false;
            continue;
        }
        
        if in_not_staged_section && line.starts_with('\t') {
            not_staged_files.push(line.trim_start_matches('\t'));
        }
    }

    not_staged_files
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
