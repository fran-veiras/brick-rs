pub fn relevant_output(job : std::process::Output, start_of_output: &str) {
    if let Ok(stdout) = String::from_utf8(job.stdout) {
        if let Some(run_finished_index) = stdout.find(&format!("{}", start_of_output)) {
            let relevant_output = &stdout[run_finished_index + start_of_output.len()..];
            println!("{}", relevant_output);
    }
    } else {
        println!("Failed to parse stdout as UTF-8");
    }
}
