use std::env;
use std::path::PathBuf;

/// 实现cd命令功能，更改当前工作目录
pub fn change_directory(path: &str) {
    // 处理特殊路径符号
    let target_path = match path {
        "~" => {
            // 对于~，尝试获取用户主目录
            match env::var(if cfg!(target_os = "windows") { "USERPROFILE" } else { "HOME" }) {
                Ok(home_dir) => PathBuf::from(home_dir),
                Err(_) => {
                    eprintln!("❌ 错误: 无法确定用户主目录");
                    return;
                }
            }
        },
        "" | "." => {
            // 空路径或.表示当前目录
            match env::current_dir() {
                Ok(current) => current,
                Err(_) => {
                    eprintln!("❌ 错误: 无法获取当前目录");
                    return;
                }
            }
        },
        ".." => {
            // ..表示上级目录
            if let Ok(current) = env::current_dir() {
                if let Some(parent) = current.parent() {
                    parent.to_path_buf()
                } else {
                    eprintln!("❌ 错误: 已经是根目录，无法再上一级");
                    return;
                }
            } else {
                eprintln!("❌ 错误: 无法获取当前目录");
                return;
            }
        },
        _ => {
            // 处理普通路径（支持相对路径和绝对路径）
            PathBuf::from(path)
        }
    };

    // 规范化路径（解析.和..等符号）
    let normalized_path = match target_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("❌ 错误: 无法规范化路径: {}", err);
            return;
        }
    };

    // 检查路径是否存在
    if !normalized_path.exists() {
        eprintln!("❌ 错误: 找不到文件或目录: {}", path);
        return;
    }

    // 检查是否是目录
    if !normalized_path.is_dir() {
        eprintln!("❌ 错误: 不是目录: {}", path);
        return;
    }

    // 尝试更改当前工作目录
    if let Err(err) = env::set_current_dir(&normalized_path) {
        eprintln!("❌ 错误: 更改目录失败: {}", err);
    } else {
        // 输出新的工作目录
        if let Ok(new_dir) = env::current_dir() {
            println!("📍 已切换到: {}", new_dir.display());
        }
    }
}