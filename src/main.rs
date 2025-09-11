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
    }
}