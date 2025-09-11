use std::env;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// 实现uname命令功能，显示系统信息
/// 模拟Linux的uname -a命令输出
pub fn print_system_info() {
    // 使用Rust标准库获取系统信息
    let os_type = "Windows";
    
    // 获取主机名
    let host_name = env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    
    // 获取Windows版本信息
    let kernel_version = match Command::new("cmd.exe").args(&["/c", "ver"]).output() {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout).to_string()
        },
        Err(_) => "Unknown".to_string()
    };
    
    // 获取处理器信息
    let cpu_info = match Command::new("wmic").args(&["cpu", "get", "name"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            output_str.lines().skip(1).next().unwrap_or("Unknown").trim().to_string()
        },
        Err(_) => "Unknown".to_string()
    };
    
    // 获取当前时间
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let seconds = dur.as_secs();
            let days = seconds / (60 * 60 * 24);
            let hours = (seconds % (60 * 60 * 24)) / (60 * 60);
            let minutes = (seconds % (60 * 60)) / 60;
            let secs = seconds % 60;
            format!("{} days, {:02}:{:02}:{:02}", days, hours, minutes, secs)
        },
        Err(_) => "Unknown".to_string()
    };
    
    // 模拟uname -a的输出格式
    println!("🖥️ {} {} {} {} {}", 
             os_type,
             host_name,
             kernel_version.trim(),
             current_time,
             cpu_info
    );
}