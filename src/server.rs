use std::io::{Read, Write}; // 移除未使用的BufRead和BufReader
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::fs::File;

// 启动HTTP服务器的函数
pub fn start_server(path: Option<String>, port: u16) {
    let directory = path.as_deref().unwrap_or(".");
    let dir_path = Path::new(directory);
    
    // 检查目录是否存在
    if !dir_path.exists() || !dir_path.is_dir() {
        eprintln!("错误: 目录 '{directory}' 不存在或不是一个有效目录");
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
    let _method = parts.next().unwrap_or(""); // 将未使用的变量method改为_method
    let path = parts.next().unwrap_or("/");
    
    // 构建文件路径
    let file_path = if path == "/" {
        root_dir.join("index.html").canonicalize().unwrap_or_else(|_| root_dir.join("index.html"))
    } else {
        let mut safe_path = PathBuf::new();
        for component in Path::new(&path[1..]).components() {
            if let std::path::Component::Normal(comp) = component {
                safe_path.push(comp);
            }
        }
        // 修复移动值问题，使用clone()或引用
        let safe_path_clone = safe_path.clone();
        root_dir.join(safe_path).canonicalize().unwrap_or_else(|_| root_dir.join(safe_path_clone))
    };
    
    // 检查文件是否存在且在根目录内
    if !file_path.exists() || !file_path.is_file() {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }
    
    // 读取文件内容
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("打开文件失败: {}", err);
            let response = "HTTP/1.1 500 Internal Server Error\r\n\r\nInternal Server Error";
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    };
    
    let mut content = Vec::new();
    if let Err(err) = file.read_to_end(&mut content) {
        eprintln!("读取文件内容失败: {}", err);
        let response = "HTTP/1.1 500 Internal Server Error\r\n\r\nInternal Server Error";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }
    
    // 发送响应
    let content_type = get_content_type(&file_path);
    let response_headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type,
        content.len()
    );
    
    stream.write_all(response_headers.as_bytes()).unwrap();
    stream.write_all(&content).unwrap();
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