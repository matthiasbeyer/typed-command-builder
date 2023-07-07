use std::path::PathBuf;

use typed_command_builder::Runnable;
use typed_command_builder::TypedCommandBuilder;

#[derive(TypedCommandBuilder)]
#[command_builder(command_name = "cat")]
struct CatFileCommand {
    #[command_builder(positional(name = "FILE"))]
    file: PathBuf,

    #[command_builder(default, arg(name = "number"))]
    show_lines: bool,
}

pub fn main() {
    let command = CatFileCommand::builder()
        .show_lines(true)
        .file(PathBuf::from(file!()))
        .build();

    let std::process::Output {
        status: _,
        stdout,
        stderr: _,
    } = command.output();

    let stdout = String::from_utf8(stdout).unwrap();
    println!("{stdout}");
}
