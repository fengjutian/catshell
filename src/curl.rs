use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

/// 执行HTTP请求并显示响应（使用Windows PowerShell）
pub fn execute_request(
    url: &str,
    method: &str,
    headers: &[(&str, &str)],
    show_headers: bool,
    silent: bool,
    data: Option<&str>,
    output: Option<&str>
) {
    // 构建PowerShell命令
    let mut ps_command = String::new();
    
    // 设置方法和URL
    ps_command.push_str(&format!("$response = Invoke-WebRequest -Uri '{}' -Method {}", url, method));
    
    // 添加请求头
    if !headers.is_empty() {
        ps_command.push_str(" -Headers @{");
        for (key, value) in headers {
            ps_command.push_str(&format!("'{}'='{}';", key, value));
        }
        ps_command.push_str("}");
    }
    
    // 添加请求体
    if let Some(body) = data {
        // 对数据进行转义，以避免PowerShell解析错误
        let escaped_body = body.replace("'", "''");
        ps_command.push_str(&format!(" -Body '{}'", escaped_body));
    }
    
    // 添加其他选项
    if !show_headers {
        ps_command.push_str(" -UseBasicParsing");
    }
    
    // 如果需要显示状态码和头部
    if show_headers || !silent {
        ps_command.push_str("\nWrite-Output ('HTTP/1.1 {0} {1}' -f $response.StatusCode, $response.StatusDescription)");
        
        if show_headers && !silent {
            ps_command.push_str("\n$response.Headers | ForEach-Object {\n");
            ps_command.push_str("    foreach ($key in $_.Keys) {\n");
            ps_command.push_str("        Write-Output ('{0}: {1}' -f $key, $_.Item($key))\n");
            ps_command.push_str("    }\n");
            ps_command.push_str("}\n");
            ps_command.push_str("Write-Output ''"); // 空行分隔头部和正文
        }
    }
    
    // 处理响应内容
    if output.is_some() {
        // 保存到文件
        let file_path = output.unwrap();
        ps_command.push_str(&format!("\n[System.IO.File]::WriteAllBytes('{}', $response.Content)", file_path));
        if !silent {
            ps_command.push_str(&format!("\nWrite-Output '✅ 响应已保存到 {}'", file_path));
        }
    } else if !silent {
        // 尝试将内容作为字符串输出（处理文本内容）
        ps_command.push_str("\ntry {\n");
        ps_command.push_str("    $content = [System.Text.Encoding]::UTF8.GetString($response.Content)\n");
        ps_command.push_str("    Write-Output $content\n");
        ps_command.push_str("} catch {\n");
        ps_command.push_str("    Write-Output '[二进制内容或无法解码]'\n");
        ps_command.push_str("}");
    }
    
    // 执行PowerShell命令
    let output = Command::new("powershell.exe")
        .arg("-Command")
        .arg(&ps_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    
    match output {
        Ok(result) => {
            // 打印标准输出
            if !result.stdout.is_empty() {
                if let Ok(stdout_str) = String::from_utf8(result.stdout) {
                    print!("{}", stdout_str);
                }
            }
            
            // 打印错误输出
            if !result.stderr.is_empty() {
                if let Ok(stderr_str) = String::from_utf8(result.stderr) {
                    eprintln!("❌ {}", stderr_str.trim());
                }
            }
        },
        Err(err) => {
            eprintln!("❌ 无法执行PowerShell命令: {}", err);
        }
    }
}

/// 获取URL的HTTP状态码
pub fn get_status_code(url: &str) {
    // 使用PowerShell发送HEAD请求
    let ps_command = format!(
        "try {{
            $response = Invoke-WebRequest -Uri '{}' -Method Head -UseBasicParsing
            Write-Output ('{{0}} - {{1}}' -f $response.StatusCode, $response.StatusDescription)
        }} catch {{
            Write-Error ('请求失败: {{0}}' -f $_.Exception.Message)
        }}",
        url
    );
    
    let output = Command::new("powershell.exe")
        .arg("-Command")
        .arg(&ps_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    
    match output {
        Ok(result) => {
            if !result.stdout.is_empty() {
                if let Ok(stdout_str) = String::from_utf8(result.stdout) {
                    println!("{}", stdout_str.trim());
                }
            }
            if !result.stderr.is_empty() {
                if let Ok(stderr_str) = String::from_utf8(result.stderr) {
                    eprintln!("❌ {}", stderr_str.trim());
                }
            }
        },
        Err(err) => {
            eprintln!("❌ 无法执行PowerShell命令: {}", err);
        }
    }
}