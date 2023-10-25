fn extract_branch_changes<'a>(
    output: &'a str,
    directories_to_run: &mut Vec<&'a str>,
    prefix: &'a str,
) {
    let lines: Vec<&str> = output.lines().collect();
    let mut in_prefix_section = false;

    for line in lines {
        if line.starts_with(prefix) {
            in_prefix_section = true;
            continue;
        } else if in_prefix_section && line.is_empty() {
            in_prefix_section = false;
            continue;
        }

        if in_prefix_section && line.starts_with('\t') {
            directories_to_run.push(line.trim_start_matches('\t'));
        }
    }
}

pub fn extract_directories(output: &str) -> Vec<&str> {
    let mut directories_to_run: Vec<&str> = Vec::new();

    [
        "Changes not staged for commit:",
        "Untracked files:",
        "Changes to be committed:",
    ]
    .iter()
    .for_each(|prefix| extract_branch_changes(output, &mut directories_to_run, prefix));

    directories_to_run
}
