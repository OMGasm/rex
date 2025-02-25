mod editor;
mod input;
mod view;
mod file;
mod panel;
use clap::Parser;
use editor::Editor;
use std::{fs::File, io, path::PathBuf};

#[derive(Parser, Debug)]
struct CliArgs {
    file: PathBuf,
    #[arg(default_value_t = 8)]
    bytes_per_group: u16,
    #[arg(default_value_t = 2)]
    groups_per_row: u16,
}

fn main() -> io::Result<()> {
    let args = CliArgs::parse();
    let file = File::open(&args.file).expect("File not found");
    let path = std::fs::canonicalize(&args.file)?;

    let mut editor = Editor::new(
        io::stdout(),
        editor::Options{
            bytes_per_group,
            groups_per_row,
            .. Default::default()
        }
    );
    editor.open_file(path)?;

    let res = editor.event_loop();

    if let Err(e) = res {
        eprintln!("{e}");
    };
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
