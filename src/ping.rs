use std::process::{Command, Output};

/// 执行ping命令
pub fn ping_host(host: &str, count: u32, timeout: u32, packet_size: u32) {
    // 根据操作系统选择合适的ping命令参数
    #[cfg(windows)]
    let output = Command::new("ping")
        .arg("-n")
        .arg(count.to_string())
        .arg("-w")
        .arg((timeout * 1000).to_string()) // Windows的-w参数是毫秒
        .arg("-l")
        .arg(packet_size.to_string())
        .arg(host)
        .output();
    
    // 这里可以添加其他操作系统的支持
    
    match output {
        Ok(output) => {
            print_ping_result(&output, host);
        },
        Err(err) => {
            eprintln!("执行ping命令失败: {}", err);
        }
    }
}

/// 打印ping命令的结果
fn print_ping_result(output: &Output, host: &str) {
    // 将输出转换为字符串
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !stderr.is_empty() {
        eprintln!("错误: {}", stderr);
        return;
    }
    
    println!("{}", stdout);
    
    // 检查是否ping成功
    if output.status.success() {
        println!("✅ {} 可访问", host);
    } else {
        println!("❌ {} 不可访问", host);
    }
}