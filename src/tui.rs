use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, 
                execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use tui::{backend::{Backend, CrosstermBackend}, 
          layout::{Alignment, Constraint, Direction, Layout}, 
          style::{Color, Modifier, Style}, 
          text::{Span, Spans}, 
          widgets::{Block, Borders, List, ListItem, Paragraph, Tabs}, 
          Terminal};

use crate::{ls, pwd, cd, mkdir, rm, cat, curl, cmatrix, vim, open_browser, open, server, ping, zip};
use std::process::Command;

/// 启动TUI界面
pub fn start_tui() -> Result<(), anyhow::Error> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建通道用于命令执行和UI更新
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    // 处理命令执行的线程
    let handle = thread::spawn(move || {
        // 这里可以处理命令执行
    });

    // 主循环
    let mut selected_tab = 0;
    let tabs = vec!["主菜单", "文件操作", "网络工具", "系统工具", "退出"];
    let mut command_output = "欢迎使用CatShell TUI!\n输入命令或使用Tab键切换功能面板".to_string();
    let mut current_dir = pwd::get_current_directory();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            
            // 绘制标题栏
            let title = Paragraph::new("CatShell 🐱 终端用户界面")
                .style(Style::default().fg(Color::Yellow).bg(Color::Black).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, size);

            // 主布局
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(20),
                    Constraint::Length(3),
                ])
                .split(size);

            // 绘制标签页
            let tabs = Tabs::new(tabs.iter().cloned().collect())
                .block(Block::default().borders(Borders::ALL))
                .select(selected_tab)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .divider(Span::raw(">"));
            f.render_widget(tabs, chunks[0]);

            // 根据选中的标签页显示不同内容
            match selected_tab {
                0 => {
                    // 主菜单
                    let items = vec![
                        ListItem::new("📁 ls - 列出目录内容"),
                        ListItem::new("📍 cd - 切换目录"),
                        ListItem::new("📝 cat - 查看文件内容"),
                        ListItem::new("🌐 curl - 发起HTTP请求"),
                        ListItem::new("🏓 ping - 测试网络连接"),
                        ListItem::new("📦 zip - 压缩/解压缩文件"),
                        ListItem::new("🖥️ uname - 显示系统信息"),
                        ListItem::new("⚡ server - 启动HTTP服务器"),
                        ListItem::new("🌈 cmatrix - 矩阵动画"),
                    ];
                    let menu = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("可用命令"))
                        .style(Style::default().fg(Color::Green))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    f.render_widget(menu, chunks[1]);
                },
                1 => {
                    // 文件操作面板
                    let items = vec![
                        ListItem::new(format!("当前目录: {}", current_dir)),
                        ListItem::new("1. ls - 列出文件"),
                        ListItem::new("2. cd <目录> - 切换目录"),
                        ListItem::new("3. mkdir <目录> - 创建目录"),
                        ListItem::new("4. rm <文件> - 删除文件/目录"),
                        ListItem::new("5. cat <文件> - 查看文件内容"),
                    ];
                    let file_panel = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("文件操作"))
                        .style(Style::default().fg(Color::Blue))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    f.render_widget(file_panel, chunks[1]);
                },
                2 => {
                    // 网络工具面板
                    let items = vec![
                        ListItem::new("1. curl <URL> - 发起HTTP请求"),
                        ListItem::new("2. ping <主机> - 测试网络连接"),
                        ListItem::new("3. server [目录] [端口] - 启动HTTP服务器"),
                        ListItem::new("4. open-browser <URL> - 打开浏览器"),
                    ];
                    let net_panel = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("网络工具"))
                        .style(Style::default().fg(Color::Magenta))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    f.render_widget(net_panel, chunks[1]);
                },
                3 => {
                    // 系统工具面板
                    let items = vec![
                        ListItem::new("1. uname - 显示系统信息"),
                        ListItem::new("2. cmatrix - 矩阵动画"),
                        ListItem::new("3. vim [文件] - 文本编辑器"),
                        ListItem::new("4. open - 打开当前目录"),
                    ];
                    let sys_panel = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("系统工具"))
                        .style(Style::default().fg(Color::Red))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    f.render_widget(sys_panel, chunks[1]);
                },
                4 => {
                    // 退出确认
                    let exit_msg = Paragraph::new("按Enter确认退出，按其他键取消")
                        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("确认退出"));
                    f.render_widget(exit_msg, chunks[1]);
                },
                _ => {},
            }

            // 绘制命令输出区域
            let output = Paragraph::new(command_output.clone())
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("输出"))
                .wrap(tui::widgets::Wrap { trim: true });
            f.render_widget(output, chunks[2]);

        })?;

        // 处理输入事件
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => {
                        selected_tab = (selected_tab + 1) % tabs.len();
                    },
                    KeyCode::BackTab => {
                        selected_tab = if selected_tab > 0 { selected_tab - 1 } else { tabs.len() - 1 };
                    },
                    KeyCode::Enter => {
                        if selected_tab == 4 {
                            // 退出确认
                            break;
                        } else {
                            command_output = "执行当前选中项\n".to_string();
                            // 这里可以添加执行选中项的逻辑
                        }
                    },
                    KeyCode::Char('q') => {
                        break;
                    },
                    _ => {},
                }
            }
        }
    }

    // 清理终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

/// 在pwd模块中添加此函数以获取当前目录
pub fn get_current_directory() -> String {
    if let Ok(path) = std::env::current_dir() {
        if let Some(path_str) = path.to_str() {
            return path_str.to_string();
        }
    }
    "未知目录".to_string()
}

/// 启动基于Windows PowerShell的TUI界面
pub fn start_tui() {
    println!("🚀 正在启动CatShell TUI...");
    
    // 使用PowerShell创建一个简单的交互式终端界面
    let ps_script = r#"
        # 设置控制台颜色和标题
        $Host.UI.RawUI.WindowTitle = "CatShell TUI 🐱"
        $Host.UI.RawUI.BackgroundColor = "Black"
        $Host.UI.RawUI.ForegroundColor = "Green"
        Clear-Host
        
        # 显示欢迎信息和命令列表
        Write-Host "==================================="
        Write-Host "    CatShell 终端用户界面"
        Write-Host "==================================="
        Write-Host ""
        Write-Host "📋 可用命令："
        Write-Host "  ls     - 列出目录内容"
        Write-Host "  cd     - 切换目录"
        Write-Host "  pwd    - 显示当前目录"
        Write-Host "  mkdir  - 创建目录"
        Write-Host "  rm     - 删除文件/目录"
        Write-Host "  cat    - 查看文件内容"
        Write-Host "  curl   - 发起HTTP请求"
        Write-Host "  ping   - 测试网络连接"
        Write-Host "  server - 启动HTTP服务器"
        Write-Host "  cmatrix - 矩阵动画"
        Write-Host ""
        Write-Host "💡 提示：输入 'exit' 退出TUI"
        Write-Host "==================================="
        Write-Host ""
        
        # 主交互循环
        while ($true) {
            # 显示提示符并获取用户输入
            Write-Host -NoNewline "cat@shell:~$ "
            $command = Read-Host
            
            # 退出TUI
            if ($command -eq "exit") {
                break
            }
            
            # 执行命令并显示结果
            try {
                Write-Host ""
                # 执行命令并格式化输出
                iex $command 2>&1 | Out-String -Stream | ForEach-Object { 
                    $_.TrimEnd()  # 移除行尾空白
                }
                Write-Host ""
            } catch {
                Write-Host "❌ 命令执行错误: $_" -ForegroundColor Red
                Write-Host ""
            }
        }
        
        # 恢复控制台设置
        $Host.UI.RawUI.BackgroundColor = "DarkMagenta"
        $Host.UI.RawUI.ForegroundColor = "Gray"
        Clear-Host
        Write-Host "感谢使用CatShell! 👋"
    "#;
    
    // 执行PowerShell脚本
    let _ = Command::new("powershell.exe")
        .arg("-Command")
        .arg(&ps_script)
        .status();
    
    println!("👋 TUI已退出");
}

// 注意：此实现依赖Windows PowerShell，无需任何外部库