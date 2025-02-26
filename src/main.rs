mod editor;
mod file;
mod input;
mod panel;
mod stuff;
mod view;
use clap::Parser;
use editor::{Editor, EditorError};
use std::{io, path::PathBuf};

#[derive(Parser, Debug)]
struct CliArgs {
    file: PathBuf,
    #[arg(default_value_t = 8)]
    bytes_per_group: u16,
    #[arg(default_value_t = 2)]
    groups_per_row: u16,
}

fn main() -> Result<(), EditorError> {
    let CliArgs {
        file: file_path,
        bytes_per_group,
        groups_per_row,
        ..
    } = CliArgs::parse();
    let path = std::fs::canonicalize(&file_path)?;

    let mut editor = Editor::new(
        io::stdout(),
        editor::Options {
            bytes_per_group,
            groups_per_row,
            ..Default::default()
        },
    );
    editor.open_file(path)?;
    editor.event_loop();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli() {
        use super::*;
        use clap::CommandFactory;

        CliArgs::command().debug_assert();
    }
}
