use std::path::Path;
use actix_files::Files;
use actix_web::{App, HttpServer, web};

// 启动HTTP服务器的函数
pub fn start_server(path: Option<String>, port: u16) {
    let directory = path.as_deref().unwrap_or(".");
    let dir_path = Path::new(directory);
    
    // 检查目录是否存在
    if !dir_path.exists() || !dir_path.is_dir() {
        eprintln!("错误: 目录 '{directory}' 不存在或不是一个有效目录");
        return;
    }
    
    let dir_str = directory.to_string();
    
    println!("正在启动HTTP服务器，服务目录: {}", dir_path.display());
    println!("服务器将在 http://localhost:{port}/ 上运行");
    println!("按Ctrl+C停止服务器");
    
    // 启动HTTP服务器
    let server = HttpServer::new(move || {
        App::new()
            .service(Files::new("/", &dir_str).show_files_listing())
    })"{port}")
    .run();
    
    // 运行服务器并处理错误
    if let Err(err) = server {
        eprintln!("启动服务器失败: {err}");
    }
}