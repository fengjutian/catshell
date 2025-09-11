use std::fs;
use std::path::Path;

/// 实现mkdir命令功能，创建目录
pub fn create_directories(paths: &[&str], parents: bool) {
    for path in paths {
        let target_path = Path::new(path);
        
        // 检查目录是否已存在
        if target_path.exists() {
            if target_path.is_dir() {
                eprintln!("❌ Error: Directory '{}' already exists", path);
            } else {
                eprintln!("❌ Error: File '{}' exists and is not a directory", path);
            }
            continue;
        }
        
        // 尝试创建目录
        let result = if parents {
            // 使用create_dir_all创建所有不存在的父目录
            fs::create_dir_all(target_path)
        } else {
            // 使用create_dir仅创建单个目录
            fs::create_dir(target_path)
        };
        
        // 处理结果
        match result {
            Ok(_) => println!("✅ Created directory: {}", path),
            Err(err) => eprintln!("❌ Error: Failed to create directory '{}': {}", path, err),
        }
    }
}