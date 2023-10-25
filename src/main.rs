use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::process::Command;
mod utils;
use utils::loader::loader;
use utils::relevant_output::relevant_output;
mod version_control;

#[derive(Serialize, Deserialize)]
struct Configfile {
    pm: String,
    root: String,
    jobs: Vec<String>,
}

fn main() -> Result<()> {
    let gitstatus = Command::new("git")
        .arg("-C")
        .arg("./")
        .arg("status")
        .output()
        .expect("ls command failed to start");

    if gitstatus.status.success() {
        ci_brick(gitstatus)
    } else {
        eprintln!(
            "Error running git command:\n{}",
            String::from_utf8_lossy(&gitstatus.stderr)
        );
        Ok(())
    }
}

fn ci_brick(gitstatus: std::process::Output) -> Result<()> {
    let stdout = String::from_utf8_lossy(&gitstatus.stdout);
    let mut package_changed = Vec::new();
    let directories_to_runv2 = version_control::directories_to_run::extract_directories(&stdout);

    let contents = fs::read_to_string("./brick.config.json").expect("Brick config was not found");
    let config_file: Configfile = serde_json::from_str(&contents)?;

    let available_files = directories_to_runv2
        .iter()
        .filter(|x| x.contains(&format!("/{}/", config_file.root)))
        .map(|x| {
            let prefixes = ["modified: ", "deleted: ", "new file: "];
            let file = prefixes
                .iter()
                .fold(x.to_string(), |acc, &prefix| {
                    acc.strip_prefix(prefix).unwrap_or(&acc).to_string()
                })
                .trim_start()
                .to_string();

            file
        })
        .collect::<Vec<String>>();

    let mut directories_to_run: Vec<String> = Vec::new();

    for file in available_files {
        let (folder, p_changed) = extract_folder_to_run(&file, &config_file);

        if !package_changed.contains(&format!("{}", p_changed)) {
            package_changed.push(format!("{}", p_changed));
        }

        if !directories_to_run.contains(&folder) {
            directories_to_run.push(folder)
        }
    }

    config_file.jobs.iter().for_each(|job| match job.as_str() {
        "cypress" => {
            package_changed.iter().for_each(|package| {
                let _loader = loader(job, package);

                let job: std::process::Output = Command::new("npx")
                    .arg("cypress")
                    .arg("run")
                    .arg("--spec")
                    .arg(format!("cypress/e2e/{}/*", package))
                    .output()
                    .expect("Failed to execute cypress");

                relevant_output(job, "(Run Finished)");
            });
        }
        _ => {
            for file in &directories_to_run {
                let _loader = loader(job, file);

                let job = Command::new(&config_file.pm)
                    .arg(job)
                    .arg("--colors")
                    .arg(&file)
                    .output()
                    .expect("something is wrong!");

                println!("{}", String::from_utf8_lossy(&job.stdout));
                println!("{}", String::from_utf8_lossy(&job.stderr));
                println!("Status: {}", job.status);
            }
        }
    });

    Ok(())
}

fn extract_folder_to_run(x: &str, config_file: &Configfile) -> (String, String) {
    let split_directories = x.split("/");

    let mut is_root = false;
    let mut folder: String = String::new();
    let mut package_changed: String = String::new();

    for part in split_directories {
        if part == config_file.root {
            folder.push_str(&format!("{}/", part));
            is_root = true;
            continue;
        }

        if is_root == true {
            folder.push_str(&format!("{}/", part));
            package_changed.push_str(&format!("{}/", part));
            break;
        }

        folder.push_str(&format!("{}/", part));
    }

    (folder, package_changed)
}
