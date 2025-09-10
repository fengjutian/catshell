use clap::{Parser, Subcommand};
use std::path::Path;

// 导入我们的命令模块
mod ls;
mod pwd;
mod rm;

/// 定义子命令枚举
#[derive(Subcommand, Debug)]
enum Commands {
    /// List directory contents
    Ls(LsCommand),
    
    /// Print working directory
    Pwd,
    
    /// Remove files or directories
    Rm(RmCommand),
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

/// A simple implementation of the rm command
#[derive(Parser, Debug)]
struct RmCommand {
    /// Files or directories to remove
    #[arg(required = true)]
    paths: Vec<String>,
    
    /// Recursively remove directories and their contents
    #[arg(short = 'r', long)]
    recursive: bool,
    
    /// Ignore nonexistent files and arguments, never prompt
    #[arg(short = 'f', long)]
    force: bool,
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
        Commands::Rm(rm_args) => {
            // 将Vec<String>转换为&[&str]用于传递给remove_files函数
            let paths_ref: Vec<&str> = rm_args.paths.iter().map(String::as_str).collect();
            rm::remove_files(&paths_ref, rm_args.recursive, rm_args.force);
        },
    }
}