use clap::{Parser, Subcommand};

mod ls;
mod pwd;
mod rm;
mod uname;
mod cd; // 添加cd模块
mod mkdir; // 添加mkdir模块

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List directory contents
    Ls {
        /// Path to list (default is current directory)
        path: Option<String>,
        
        /// Show hidden files
        #[arg(short, long)]
        all: bool,
        
        /// Long format listing
        #[arg(short, long)]
        long: bool,
        
        /// List subdirectories recursively
        #[arg(short, long)]
        recursive: bool,
    },
    
    /// Print working directory
    Pwd,
    
    /// Remove files or directories
    Rm {
        /// Files or directories to remove
        paths: Vec<String>,
        
        /// Remove directories and their contents recursively
        #[arg(short, long)]
        recursive: bool,
        
        /// Ignore nonexistent files and arguments, never prompt
        #[arg(short, long)]
        force: bool,
    },
    
    /// Print system information
    Uname {
        /// Print all system information
        #[arg(short, long)]
        all: bool,
    },
    
    /// Change directory
    Cd {
        /// Directory to change to
        path: String,
    },
    
    /// Make directories
    Mkdir {
        /// Directories to create
        paths: Vec<String>,
        
        /// Create parent directories as needed
        #[arg(short, long)]
        parents: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Ls { path, all, long, recursive } => {
            let path = path.as_deref().unwrap_or(".");
            ls::list_directory(
                std::path::Path::new(path), 
                *all, 
                *long, 
                *recursive
            );
        },
        
        Commands::Pwd => {
            pwd::print_working_directory();
        },
        
        Commands::Rm { paths, recursive, force } => {
            let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
            rm::remove_files(&path_refs, *recursive, *force);
        },
        
        Commands::Uname { all: _ } => { // 使用_忽略未使用的变量
            // 目前我们只实现了-a选项的功能
            uname::print_system_info();
        },
        
        Commands::Cd { path } => {
            cd::change_directory(path);
        },
        
        Commands::Mkdir { paths, parents } => {
            let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
            mkdir::create_directories(&path_refs, *parents);
        },
    }
}