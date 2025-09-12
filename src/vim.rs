use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

/// 实现简化版的vim编辑器
pub fn run_vim(file_path: Option<&str>) {
    // 打印提示信息
    println!("🐱 CatShell Vim (简化版)");
    println!("💡 提示: 这是一个简化版vim，仅支持基本功能");
    println!("💾 保存并退出: 使用编辑器自带的保存功能");
    println!("🚪 退出: 关闭编辑器窗口");
    println!("⏳ 正在启动编辑器...");

    // 在Windows上，我们使用内置的notepad作为编辑器
    let editor_path = if cfg!(target_os = "windows") {
        "notepad.exe"
    } else {
        "vi" // 非Windows系统尝试使用vi
    };

    let mut cmd = Command::new(editor_path);
    
    // 如果提供了文件路径，打开该文件
    if let Some(path) = file_path {
        cmd.arg(path);
    }

    // 设置标准输入、输出和错误
    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    // 执行命令
    match cmd.spawn() {
        Ok(mut child) => {
            // 等待编辑器退出
            if let Err(err) = child.wait() {
                eprintln!("❌ 编辑器执行失败: {}", err);
            }
        },
        Err(err) => {
            eprintln!("❌ 无法启动编辑器: {}", err);
            // 提供一个更基础的文件编辑选项
            if let Some(path) = file_path {
                basic_file_edit(path);
            } else {
                eprintln!("💡 尝试使用 `cat` 命令查看文件内容");
            }
        }
    }
}

/// 非常基础的文件编辑功能，作为备用方案
fn basic_file_edit(file_path: &str) {
    println!("📝 使用基础编辑器编辑文件: {}", file_path);
    println!("💡 输入内容，按Ctrl+Z然后Enter保存退出");
    
    // 读取现有文件内容
    let mut content = String::new();
    if let Ok(mut file) = File::open(file_path) {
        if let Ok(_) = file.read_to_string(&mut content) {
            println!("{}", content);
        }
    }
    
    // 提示用户输入
    println!("\n===== 编辑模式 (Ctrl+Z+Enter 保存退出) =====");
    
    // 读取用户输入
    let mut new_content = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut new_content) {
        // 保存文件
        if let Ok(mut file) = File::create(file_path) {
            if let Ok(_) = file.write_all(new_content.as_bytes()) {
                println!("✅ 文件已保存: {}", file_path);
            } else {
                eprintln!("❌ 无法写入文件: {}", file_path);
            }
        } else {
            eprintln!("❌ 无法创建文件: {}", file_path);
        }
    }
}