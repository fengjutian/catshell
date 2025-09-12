use std::env;
use std::process::Command;

/// 打开当前目录（在Windows上使用资源管理器）
pub fn open_current_directory() {
    // 获取当前工作目录
    match env::current_dir() {
        Ok(current_dir) => {
            println!("正在打开目录: {}", current_dir.display());
            
            // 在Windows上使用explorer.exe打开目录
            let result = Command::new("explorer.exe")
                .arg(current_dir)
                .spawn();
            
            match result {
                Ok(_) => {
                    println!("目录已成功打开");
                },
                Err(err) => {
                    eprintln!("无法打开目录: {}", err);
                }
            }
        },
        Err(err) => {
            eprintln!("无法获取当前工作目录: {}", err);
        }
    }
}


