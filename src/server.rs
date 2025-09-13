use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

// 启动HTTP服务器的函数
pub fn start_server(path: Option<String>, port: u16) {
    let directory = path.as_deref().unwrap_or(".");
    let dir_path = Path::new(directory);
    
    // 检查目录是否存在
    if !dir_path.exists() || !dir_path.is_dir() {
        eprintln!("错误: 目录 '{}' 不存在或不是一个有效目录", directory);
        return;
    }
    
    let address = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("无法绑定到地址 {}: {}", address, err);
            return;
        }
    };
    
    println!("正在启动HTTP服务器，服务目录: {}", dir_path.display());
    println!("服务器将在 http://localhost:{}/ 上运行", port);
    println!("注意：此实现无法通过Ctrl+C停止，请手动关闭终端窗口来停止服务器");
    
    // 接受连接并处理请求
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let dir_path = dir_path.to_path_buf();
                thread::spawn(move || {
                    handle_client(stream, &dir_path);
                });
            }
            Err(err) => {
                eprintln!("接受连接失败: {}", err);
            }
        }
    }
}

// 处理客户端请求
fn handle_client(mut stream: TcpStream, root_dir: &Path) {
    let mut buffer = [0; 1024];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(err) => {
            eprintln!("读取请求失败: {}", err);
            return;
        }
    };
    
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or("");
    
    // 解析请求行
    let mut parts = request_line.split_whitespace();
    let _method = parts.next().unwrap_or(""); // 忽略HTTP方法
    let path = parts.next().unwrap_or("/");
    
    // 构建文件路径
    let file_path = build_file_path(root_dir, path);
    
    // 发送文件或404响应
    if file_path.exists() && file_path.is_file() {
        send_file(&mut stream, &file_path);
    } else {
        send_404(&mut stream);
    }
}

// 构建文件路径
fn build_file_path(root_dir: &Path, request_path: &str) -> PathBuf {
    // 处理根路径请求
    if request_path == "/" {
        // 尝试查找index.html, index.htm, 或显示目录列表
        let index_html = root_dir.join("index.html");
        let index_htm = root_dir.join("index.htm");
        
        if index_html.exists() {
            return index_html;
        } else if index_htm.exists() {
            return index_htm;
        }
        // 如果没有index文件，返回一个特殊标记表示需要显示目录列表
        return root_dir.join("__directory_listing__");
    }
    
    // 防止目录遍历攻击
    let safe_path = sanitize_path(request_path);
    root_dir.join(safe_path)
}

// 清理路径，防止目录遍历攻击
fn sanitize_path(path: &str) -> String {
    let mut result = String::new();
    let parts: Vec<&str> = path.split('/').filter(|&p| !p.is_empty() && p != "..").collect();
    
    for part in parts {
        if !result.is_empty() {
            result.push('/');
        }
        result.push_str(part);
    }
    
    result
}

// 发送文件内容
fn send_file(stream: &mut TcpStream, file_path: &Path) {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = Vec::new();
            if file.read_to_end(&mut content).is_ok() {
                let content_type = get_content_type(file_path);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    content_type,
                    content.len()
                );
                
                stream.write_all(response.as_bytes()).ok();
                stream.write_all(&content).ok();
            } else {
                send_500(stream);
            }
        },
        Err(_) => {
            send_500(stream);
        }
    }
}

// 发送404响应
fn send_404(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n"
        .to_string() + 
        "<html><body><h1>404 Not Found</h1><p>请求的资源不存在</p></body></html>";
    stream.write_all(response.as_bytes()).ok();
}

// 发送500响应
fn send_500(stream: &mut TcpStream) {
    let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\n\r\n"
        .to_string() + 
        "<html><body><h1>500 Internal Server Error</h1><p>服务器内部错误</p></body></html>";
    stream.write_all(response.as_bytes()).ok();
}

// 获取文件的MIME类型
fn get_content_type(path: &Path) -> &str {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            match ext_str.to_lowercase().as_str() {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                "txt" => "text/plain",
                _ => "application/octet-stream",
            }
        } else {
            "application/octet-stream"
        }
    } else {
        "application/octet-stream"
    }
}