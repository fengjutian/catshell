use clap::{Parser, Subcommand};
use std::path::Path;

// 导入我们的命令模块
mod ls;
mod pwd;

/// 定义子命令枚举
#[derive(Subcommand, Debug)]
enum Commands {
    /// List directory contents
    Ls(LsCommand),
    
    /// Print working directory
    Pwd,
}

/// A simple implementation of the ls command
#[derive(Parser, Debug)]
struct LsCommand {
    /// Directory to list (default: current directory)
    #[arg(default_value = ".")]
    directory: String,

    /// Show hidden files
    #[arg(short, long)]
    all: bool,
    
    /// Long format listing
    #[arg(short = 'l', long)]
    long: bool,
    
    /// Recursive listing
    #[arg(short, long)]
    recursive: bool,
}

/// A simple shell implementation with basic commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CatShell {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = CatShell::parse();
    
    match args.command {
        Commands::Ls(ls_args) => {
            let path = Path::new(&ls_args.directory);
            println!("{}/:", path.display());
            ls::list_directory(path, ls_args.all, ls_args.long, ls_args.recursive);
        },
        Commands::Pwd => {
            pwd::print_working_directory();
        },
    }
}