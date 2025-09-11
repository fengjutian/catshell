use std::error::Error;
use reqwest::blocking::Client;
use reqwest::{StatusCode, Method};

/// 执行HTTP请求并显示响应
pub fn execute_request(
    url: &str,
    method: &str,
    headers: &[(&str, &str)],
    show_headers: bool,
    silent: bool,
    data: Option<&str>,
    output: Option<&str>
) {
    // 创建HTTP客户端
    let client = Client::new();
    
    // 构建请求
    let mut request_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "HEAD" => client.head(url),
        _ => {
            eprintln!("❌ 不支持的HTTP方法: {}", method);
            return;
        }
    };
    
    // 添加请求头
    for (key, value) in headers {
        request_builder = request_builder.header(*key, *value);
    }
    
    // 添加请求体
    if let Some(body) = data {
        request_builder = request_builder.body(body.to_string());
    }
    
    // 发送请求
    match request_builder.send() {
        Ok(response) => {
            // 显示状态码
            if !silent {
                println!("HTTP/1.1 {} {}", 
                         response.status().as_u16(), 
                         response.status().canonical_reason().unwrap_or("Unknown"));
            }
            
            // 显示响应头
            if show_headers && !silent {
                for (name, value) in response.headers() {
                    println!("{}: {:?}", name, value.to_str().unwrap_or("[无法解码]"));
                }
                if !silent {
                    println!(); // 空行分隔头部和正文
                }
            }
            
            // 获取响应体
            if let Ok(body) = response.text() {
                match output {
                    Some(file_path) => {
                        // 保存到文件
                        if let Err(err) = std::fs::write(file_path, &body) {
                            eprintln!("❌ 无法写入文件 {}: {}", file_path, err);
                        } else {
                            if !silent {
                                println!("✅ 响应已保存到 {}", file_path);
                            }
                        }
                    },
                    None if !silent => {
                        // 打印到控制台
                        print!("{}", body);
                    },
                    _ => {}
                }
            }
        },
        Err(err) => {
            eprintln!("❌ 请求失败: {}", err);
        }
    }
}

/// 获取URL的HTTP状态码
pub fn get_status_code(url: &str) {
    let client = Client::new();
    
    match client.head(url).send() {
        Ok(response) => {
            let status = response.status();
            println!("{} - {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
        },
        Err(err) => {
            eprintln!("❌ 请求失败: {}", err);
        }
    }
}