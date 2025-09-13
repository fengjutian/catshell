use clap::{Parser, Subcommand};

mod ls;
mod pwd;
mod rm;
mod uname;
mod cd; // 添加cd模块
mod mkdir; // 添加mkdir模块
mod cat; // 添加cat模块
mod curl; // 添加curl模块
mod cmatrix; // 添加cmatrix模块
mod vim; // 添加vim模块
mod open_browser; // 添加open_browser模块
mod open; // 添加open模块
mod server; // 添加server模块
mod ping; // 添加ping模块
mod zip; // 添加zip模块

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List directory contents
    Ls {
        /// Path to list (default is current directory)
        path: Option<String>,
        
        /// Show hidden files
        #[arg(short, long)]
        all: bool,
        
        /// Long format listing
        #[arg(short, long)]
        long: bool,
        
        /// List subdirectories recursively
        #[arg(short, long)]
        recursive: bool,
    },
    
    /// Print working directory
    Pwd,
    
    /// Remove files or directories
    Rm {
        /// Files or directories to remove
        paths: Vec<String>,
        
        /// Remove directories and their contents recursively
        #[arg(short, long)]
        recursive: bool,
        
        /// Ignore nonexistent files and arguments, never prompt
        #[arg(short, long)]
        force: bool,
    },
    
    /// Print system information
    Uname {
        /// Print all system information
        #[arg(short, long)]
        all: bool,
    },
    
    /// Change directory
    Cd {
        /// Directory to change to
        path: String,
    },
    
    /// Make directories
    Mkdir {
        /// Directories to create
        paths: Vec<String>,
        
        /// Create parent directories as needed
        #[arg(short, long)]
        parents: bool,
    },
    
    /// Concatenate and display files
    Cat {
        /// Files to display
        paths: Vec<String>,
        
        /// Number all output lines
        #[arg(short = 'n', long)]
        number_lines: bool,
        
        /// Number nonempty output lines
        #[arg(short = 'b', long)]
        number_nonblank: bool,
    },
    
    /// 发起HTTP请求 (curl-like)
    Curl {
        /// 请求的URL
        url: String,
        
        /// HTTP方法 (GET, POST, PUT, DELETE, HEAD等)
        #[arg(short = 'X', long, default_value = "GET")] // 添加明确的短选项
        request: String,
        
        /// 显示响应头
        #[arg(short = 'i', long)] // 添加明确的短选项
        include: bool,
        
        /// 不输出任何内容
        #[arg(short = 's', long)]
        silent: bool,
        
        /// 发送的数据
        #[arg(short = 'd', long)] // 添加明确的短选项
        data: Option<String>,
        
        /// 保存响应到文件
        #[arg(short = 'o', long)] // 添加明确的短选项
        output: Option<String>,
        
        /// 只显示HTTP状态码
        #[arg(long)]
        head: bool,
        
        /// 添加自定义请求头
        #[arg(short = 'H', long, num_args(1..), value_parser = parse_header)] // 将短选项改为'H'
        header: Vec<(String, String)>,
    },
    
    /// 显示矩阵风格的字符下落动画
    /// 显示矩阵风格的字符下落动画
    Cmatrix {
        /// 设置字符颜色 (green, red, blue, yellow, cyan, magenta, white)
        #[arg(short, long, default_value = "green")]
        color: String,
        
        /// 设置动画速度 (1-100, 值越小越快)
        #[arg(short, long, default_value = "10")]
        speed: u64,
        
        /// 设置字符密度 (1-100)
        #[arg(short = 'd', long, default_value = "30")]
        density: u8,
        
        /// 不使用粗体字符
        #[arg(long)]
        no_bold: bool,
    },
    
    /// 简化版vim文本编辑器
    Vim {
        /// 要编辑的文件路径（可选）
        file: Option<String>,
    },
    
    /// 打开浏览器访问指定URL
    #[command(name = "open-browser")]
    OpenBrowser {
        /// 要访问的URL地址
        url: String,
    },
    Open,
    /// 启动HTTP服务器运行HTML文件
    Server {
        /// 要提供服务的目录路径（默认是当前目录）
        path: Option<String>,
        
        /// 服务器端口号（默认8000）
        #[arg(short, long, default_value = "8000")]
        port: u16,
    },
    
    /// 向指定主机发送ICMP回显请求
    Ping {
        /// 要ping的主机名或IP地址
        host: String,
        
        /// 发送的回显请求数量
        #[arg(short = 'c', long, default_value = "4")]
        count: u32,
        
        /// 超时时间（秒）
        #[arg(short = 't', long, default_value = "4")]
        timeout: u32,
        
        /// 数据包大小（字节）
        #[arg(short = 's', long, default_value = "32")]
        size: u32,
    },
    
    /// 创建或提取zip压缩文件
    Zip {
        /// 创建压缩文件
        #[arg(short, long)]
        create: bool,
        
        /// 提取压缩文件
        #[arg(short, long)]
        extract: bool,
        
        /// 输出文件或目录路径
        #[arg(short, long)]
        output: String,
        
        /// 要压缩的文件或目录列表（创建模式）
        files: Vec<String>,
    },
}

// 解析HTTP头的辅助函数
fn parse_header(s: &str) -> Result<(String, String), String> {
    if let Some((key, value)) = s.split_once(":") {
        Ok((key.trim().to_string(), value.trim().to_string()))
    } else {
        Err("格式错误: 请使用 'Key: Value' 格式".to_string())
    }
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Ls { path, all, long, recursive } => {
            let path = path.as_deref().unwrap_or(".");
            ls::list_directory(
                std::path::Path::new(path), 
                *all, 
                *long, 
                *recursive
            );
        },
        
        Commands::Pwd => {
            pwd::print_working_directory();
        },
        
        Commands::Rm { paths, recursive, force } => {
            let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
            rm::remove_files(&path_refs, *recursive, *force);
        },
        
        Commands::Uname { all: _ } => { // 使用_忽略未使用的变量
            // 目前我们只实现了-a选项的功能
            uname::print_system_info();
        },
        
        Commands::Cd { path } => {
            cd::change_directory(path);
        },
        
        Commands::Mkdir { paths, parents } => {
            let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
            mkdir::create_directories(&path_refs, *parents);
        },
        
        Commands::Cat { paths, number_lines, number_nonblank } => {
            let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
            cat::display_files(&path_refs, *number_lines, *number_nonblank);
        },
        Commands::Curl { url, request, include, silent, data, output, head, header } => {
            // 转换header为&[(&str, &str)]格式
            let header_refs: Vec<(&str, &str)> = 
                header.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                
            if *head {
                curl::get_status_code(url);
            } else {
                curl::execute_request(
                    url,
                    request,
                    &header_refs,
                    *include,
                    *silent,
                    data.as_deref(),
                    output.as_deref()
                );
            }
        },
        Commands::Cmatrix { color, speed, density, no_bold } => {
            // 确保参数在有效范围内
            // 使用适当的类型转换修复clamp方法调用
            let adjusted_speed = speed.clamp(&1, &100);
            let adjusted_density = density.clamp(&1, &100);
            
            // 添加解引用操作符*来修复类型不匹配问题
            cmatrix::run_cmatrix(color, *adjusted_speed, *adjusted_density, *no_bold);
        },
        Commands::Vim { file } => {
            vim::run_vim(file.as_deref());
        },
        
        Commands::OpenBrowser { url } => {
            open_browser::open_browser(url);
        },
        Commands::Open => {
            open::open_current_directory();
        },
            Commands::Server { path, port } => {
            server::start_server(path.clone(), *port);
        },
        
        Commands::Ping { host, count, timeout, size } => {
            ping::ping_host(host, *count, *timeout, *size);
        },
        
        Commands::Zip { create, extract, output, files } => {
            if *create && *extract {
                eprintln!("错误: 不能同时使用 --create 和 --extract 选项");
            } else if *create {
                zip::create_zip(files, output);
            } else if *extract {
                if let Some(zip_file) = files.first() {
                    zip::extract_zip(zip_file, output);
                } else {
                    eprintln!("错误: 解压缩模式需要指定zip文件");
                }
            } else {
                eprintln!("错误: 必须指定 --create 或 --extract 选项");
            }
        },
    }
}
