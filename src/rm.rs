use std::fs;
use std::path::Path;

/// 实现rm命令功能，删除文件或目录
/// 
/// # 参数
/// - `paths`: 要删除的文件或目录路径列表
/// - `recursive`: 是否递归删除目录及其内容
/// - `force`: 是否强制删除，忽略不存在的文件和权限错误
pub fn remove_files(paths: &[&str], recursive: bool, force: bool) { 
    for path_str in paths {
        let path = Path::new(path_str);
        
        if !path.exists() {
            if !force {
                eprintln!("❌ Error: '{}' does not exist", path_str);
            }
            continue;
        }
        
        if path.is_file() {
            // 删除文件
            if let Err(err) = fs::remove_file(path) {
                if !force {
                    eprintln!("❌ Error removing file '{}': {}", path_str, err);
                }
            } else {
                if !force {
                    println!("✅ Removed file '{}'", path_str);
                }
            }
        } else if path.is_dir() {
            // 删除目录
            if recursive {
                if let Err(err) = fs::remove_dir_all(path) {
                    if !force {
                        eprintln!("❌ Error removing directory '{}': {}", path_str, err);
                    }
                } else {
                    if !force {
                        println!("✅ Removed directory '{}' and all its contents", path_str);
                    }
                }
            } else {
                // 非递归模式下，目录必须为空才能删除
                if let Err(err) = fs::remove_dir(path) {
                    eprintln!("❌ Error removing directory '{}': {}", path_str, err);
                    eprintln!("Use -r or --recursive to remove non-empty directories");
                } else {
                    if !force {
                        println!("✅ Removed directory '{}'", path_str);
                    }
                }
            }
        }
    }
}