use clap::Parser;
use std::fs;
use std::path::Path;

/// A simple implementation of the ls command
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct LsCommand {
    /// Directory to list (default: current directory)
    #[arg(default_value = ".")]
    directory: String,

    /// Show hidden files
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = LsCommand::parse();
    let path = Path::new(&args.directory);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Skip hidden files unless -a is specified
                if !args.all && file_name_str.starts_with('.') {
                    continue;
                }

                println!("{}", file_name_str);
            }
        }
    } else {
        eprintln!("Error: Could not read directory '{}'", args.directory);
        std::process::exit(1);
    }
}