use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// 实现cat命令功能，查看文件内容
pub fn display_files(paths: &[&str], number_lines: bool, number_nonblank: bool) {
    if paths.is_empty() {
        // 如果没有提供文件路径，则从标准输入读取
        read_from_stdin(number_lines, number_nonblank);
        return;
    }
    
    let mut file_count = 0;
    
    for path in paths {
        let file_path = Path::new(path);
        
        // 检查文件是否存在
        if !file_path.exists() {
            eprintln!("❌ Error: No such file or directory: {}", path);
            continue;
        }
        
        // 检查是否是文件
        if !file_path.is_file() {
            eprintln!("❌ Error: Not a file: {}", path);
            continue;
        }
        
        // 如果有多个文件，显示文件名
        if paths.len() > 1 {
            if file_count > 0 {
                println!("\n");
            }
            println!("====> {} <====", path);
            file_count += 1;
        }
        
        // 读取文件内容
        match fs::read_to_string(file_path) {
            Ok(content) => {
                print_content(&content, number_lines, number_nonblank);
            },
            Err(err) => {
                // 尝试以二进制模式读取
                match fs::read(file_path) {
                    Ok(bytes) => {
                        println!("📄 Binary file '{}' ({} bytes)", path, bytes.len());
                    },
                    Err(_) => {
                        eprintln!("❌ Error: Failed to read file '{}': {}", path, err);
                    }
                }
            }
        }
    }
}

/// 从标准输入读取内容
fn read_from_stdin(number_lines: bool, number_nonblank: bool) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    
    if let Ok(_) = handle.read_to_string(&mut buffer) {
        print_content(&buffer, number_lines, number_nonblank);
    } else {
        eprintln!("❌ Error: Failed to read from stdin");
    }
}

/// 打印内容，支持行号显示
fn print_content(content: &str, number_lines: bool, number_nonblank: bool) {
    if !number_lines && !number_nonblank {
        // 不显示行号，直接打印内容
        print!("{}", content);
        return;
    }
    
    let mut line_number = 1;
    
    for line in content.lines() {
        if number_nonblank {
            // 只对非空行显示行号
            if !line.trim().is_empty() {
                println!("{:6}  {}", line_number, line);
                line_number += 1;
            } else {
                println!();
            }
        } else {
            // 对所有行显示行号
            println!("{:6}  {}", line_number, line);
            line_number += 1;
        }
    }
}