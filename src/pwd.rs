use std::env;

/// 实现pwd命令功能，显示当前工作目录
pub fn print_working_directory() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        eprintln!("Error: Could not determine current working directory");
    }
}