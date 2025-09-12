use std::process::Command;

/// 打开浏览器访问指定的URL
pub fn open_browser(url: &str) {
    println!("正在打开浏览器访问: {}", url);
    
    // 在Windows上使用start命令打开默认浏览器
    let result = Command::new("cmd")
        .args(["/c", "start", url])
        .spawn();
    
    match result {
        Ok(_) => {
            println!("浏览器已成功打开");
        },
        Err(err) => {
            eprintln!("无法打开浏览器: {}", err);
        }
    }
}
