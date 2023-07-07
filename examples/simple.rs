use std::path::PathBuf;

use typed_command_builder::TypedCommandBuilder;

#[derive(TypedCommandBuilder)]
struct CatFileCommand {
    #[typed_command(positional, name = "FILE")]
    file: PathBuf,

    #[typed_command(name = "--number")]
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
