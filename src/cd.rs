use std::env;
use std::path::PathBuf;

/// 实现cd命令功能，更改当前工作目录
pub fn change_directory(path: &str) {
    let target_path = if path == "~" {
        // 对于~，尝试获取用户主目录
        match env::var("USERPROFILE") {
            Ok(home_dir) => PathBuf::from(home_dir),
            Err(_) => {
                eprintln!("❌ Error: Could not determine home directory");
                return;
            }
        }
    } else {
        PathBuf::from(path)
    };

    // 检查路径是否存在
    if !target_path.exists() {
        eprintln!("❌ Error: No such file or directory: {}", path);
        return;
    }

    // 检查是否是目录
    if !target_path.is_dir() {
        eprintln!("❌ Error: Not a directory: {}", path);
        return;
    }

    // 尝试更改当前工作目录
    if let Err(err) = env::set_current_dir(&target_path) {
        eprintln!("❌ Error: Failed to change directory: {}", err);
    } else {
        // 输出新的工作目录
        if let Ok(new_dir) = env::current_dir() {
            println!("📍 Changed directory to: {}", new_dir.display());
        }
    }
}