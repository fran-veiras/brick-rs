use spinner::{SpinnerBuilder, SpinnerHandle};

pub fn loader(job: &String, directory: &String) -> SpinnerHandle {
    let sp = SpinnerBuilder::new(
        format!(
            "\x1b[32m [brick-rs]: process {} in {} \x1b[0m",
            job, directory
        )
        .into(),
    )
    .spinner(vec![
        "\x1b[32m⣾\x1b[0m",
        "\x1b[32m⣽\x1b[0m",
        "\x1b[32m⣻\x1b[0m",
        "\x1b[32m⢿\x1b[0m",
        "\x1b[32m⡿\x1b[0m",
        "\x1b[32m⣟\x1b[0m",
        "\x1b[32m⣯\x1b[0m",
        "\x1b[32m⣷\x1b[0m",
        "\x1b[32m⠁\x1b[0m",
        "\x1b[32m⠂\x1b[0m",
        "\x1b[32m⠄\x1b[0m",
        "\x1b[32m⡀\x1b[0m",
        "\x1b[32m⢀\x1b[0m",
        "\x1b[32m⠠\x1b[0m",
        "\x1b[32m⠐\x1b[0m",
        "\x1b[32m⠈\x1b[0m",
    ])
    .start();

    sp
}
