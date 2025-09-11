use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime; // 移除未使用的Duration导入

// 定义文件类型枚举
#[allow(dead_code)]
pub enum FileType {
    Directory,
    File,
    Symlink,
    Other,
}

// 定义文件信息结构体
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub modified: SystemTime,
    pub is_hidden: bool,
}

// 从路径获取文件信息
pub fn get_file_info(path: &Path) -> Option<FileInfo> {
    if let Ok(metadata) = fs::metadata(path) {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_file() {
            FileType::File
        } else {
            FileType::Other
        };
        
        let is_hidden = name.starts_with('.');
        
        return Some(FileInfo {
            path: path.to_path_buf(),
            name,
            file_type,
            size: metadata.len(),
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            is_hidden,
        });
    }
    None
}

// 简化的时间格式化函数，使用标准库实现
pub fn format_time_simple(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(dur) => {
            // 获取总秒数
            let seconds = dur.as_secs();
            
            // 简单地返回时间戳字符串
            format!("{}", seconds)
        },
        Err(_) => "Unknown".to_string(),
    }
}

// 列出目录内容
pub fn list_directory(path: &Path, show_hidden: bool, long_format: bool, recursive: bool) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut file_infos = Vec::new();
        
        // 收集文件信息
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(info) = get_file_info(&entry.path()) {
                    if show_hidden || !info.is_hidden {
                        file_infos.push(info);
                    }
                }
            }
        }
        
        // 按名称排序
        file_infos.sort_by(|a, b| a.name.cmp(&b.name));
        
        // 显示文件信息
        for info in file_infos {
            // 根据文件类型选择适当的emoji
            let file_emoji = match info.file_type {
                FileType::Directory => "📁",
                FileType::File => "📄",
                FileType::Symlink => "🔗",
                FileType::Other => "❓",
            };
            
            if long_format {
                // 长格式显示
                let size_str = format!("{:10}", info.size);
                let time_str = format_time_simple(info.modified);
                let type_str = match info.file_type {
                    FileType::Directory => "d",
                    FileType::File => "-",
                    FileType::Symlink => "l",
                    FileType::Other => "?",
                };
                
                // 在Windows上我们无法直接获取UNIX风格的权限
                // 这里简化处理
                let permissions = format!("{:10}", "rwxrwxrwx");
                
                // 修复格式说明符数量，添加一个额外的{}
                println!("{}{} {} {} {} {}", file_emoji, type_str, permissions, size_str, time_str, info.name);
            } else {
                // 简单格式显示
                println!("{} {}", file_emoji, info.name);
            }
            
            // 递归显示子目录
            if recursive {
                if let FileType::Directory = info.file_type {
                    println!("\n{}/:", info.name);
                    list_directory(&info.path, show_hidden, long_format, recursive);
                }
            }
        }
    } else {
        eprintln!("❌ Error: Could not read directory '{}'", path.display());
    }
}