use std::env;

/// 实现pwd命令功能，显示当前工作目录
pub fn print_working_directory() {
    if let Ok(path) = env::current_dir() {
        if let Some(path_str) = path.to_str() {
            println!("{} 位于: {}", "📍", path_str);
        } else {
            eprintln!("❌ 无法将当前目录转换为字符串");
        }
    } else {
        eprintln!("❌ 无法获取当前工作目录");
    }
}

/// 获取当前目录路径字符串，用于TUI显示
pub fn get_current_directory() -> String {
    if let Ok(path) = env::current_dir() {
        if let Some(path_str) = path.to_str() {
            return path_str.to_string();
        }
    }
    "未知目录".to_string()
}