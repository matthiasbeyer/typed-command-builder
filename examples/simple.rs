use std::path::PathBuf;

use typed_command_builder::TypedCommandBuilder;

#[derive(TypedCommandBuilder)]
struct CatFileCommand {
    #[command_builder(positional, name = "FILE")]
    file: PathBuf,

    #[command_builder(default, name = "number")]
    show_lines: bool,
}

pub fn main() {
    let command = CatFileCommand::builder()
        .show_lines(true)
        .file(PathBuf::from(file!()))
        .build();

    let (stdout, stderr) = command.run();

    println!("{stdout}");
}
