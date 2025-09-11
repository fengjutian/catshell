// 移除未使用的self导入
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;

// cmatrix命令实现
pub fn run_cmatrix(color: &str, speed: u64, density: u8, no_bold: bool) {
    // 检查终端是否支持ANSI转义序列
    if !is_ansi_supported() {
        println!("错误: 当前终端不支持ANSI转义序列，无法运行cmatrix");
        return;
    }

    // 尝试获取终端大小
    let (width, height) = get_terminal_size().unwrap_or((80, 25));
    let mut rng = rand::thread_rng();

    // 创建矩阵列
    let mut columns: Vec<Column> = Vec::new();
    for i in 0..width {
        // 随机决定列是否激活
        if rng.gen_ratio(density as u32, 100) {
            let length = rng.gen_range(5..15);
            let speed_factor = rng.gen_range(1..4);
            columns.push(Column::new(i, length, speed_factor));
        }
    }

    // 设置颜色
    let color_code = match color.to_lowercase().as_str() {
        "green" => "\x1b[32m",
        "red" => "\x1b[31m",
        "blue" => "\x1b[34m",
        "yellow" => "\x1b[33m",
        "cyan" => "\x1b[36m",
        "magenta" => "\x1b[35m",
        "white" => "\x1b[37m",
        _ => "\x1b[32m", // 默认绿色
    };

    let bold_code = if no_bold { "" } else { "\x1b[1m" };
    let reset_code = "\x1b[0m";
    let clear_screen = "\x1b[2J\x1b[H";
    let hide_cursor = "\x1b[?25l";
    let show_cursor = "\x1b[?25h";

    // 显示欢迎信息并提示退出方法
    println!("{}CatShell Matrix 动画\n按 Ctrl+C 退出...{}", bold_code, reset_code);
    sleep(Duration::from_secs(1));

    // 进入动画循环
    let stdout = stdout();
    let mut stdout_lock = stdout.lock();
    let frame_duration = Duration::from_millis(speed);

    // 尝试隐藏光标
    write!(stdout_lock, "{}", hide_cursor).unwrap();
    stdout_lock.flush().unwrap();

    let mut last_frame = Instant::now();

    // 主循环
    loop {
        // 控制帧率
        let elapsed = last_frame.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
        last_frame = Instant::now();

        // 清屏并开始新帧
        write!(stdout_lock, "{}", clear_screen).unwrap();

        // 更新所有列
        for column in &mut columns {
            column.update(height, &mut rng);
        }

        // 绘制所有字符
        for y in 0..height {
            for column in &columns {
                if let Some(c) = column.get_char(y) {
                    // 移动到指定位置并输出字符
                    write!(stdout_lock, "\x1b[{};{}H{}{}{}", 
                           y + 1, column.x + 1, color_code, bold_code, c).unwrap();
                }
            }
        }

        stdout_lock.flush().unwrap();

        // 检查是否有退出信号（简化处理，实际Ctrl+C会通过信号处理）
        if check_for_exit() {
            break;
        }
    }

    // 恢复终端状态
    write!(stdout_lock, "{}{}{}", reset_code, clear_screen, show_cursor).unwrap();
    stdout_lock.flush().unwrap();
}

// 定义矩阵列结构
struct Column {
    x: u16,              // 列的X坐标
    characters: Vec<char>, // 列中的字符
    positions: Vec<u16>, // 字符的Y坐标
    speed: u8,           // 下落速度因子
    counter: u8,         // 速度计数器
}

impl Column {
    fn new(x: u16, length: u8, speed: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut characters = Vec::new();
        let mut positions = Vec::new();
        
        // 随机起始位置
       let random_offset = rng.gen_range(0..30);
        let start_y = (random_offset as i16).saturating_sub(30); // 使用saturating_sub避免下溢
        
        // 填充字符
              for i in 0..length {
            characters.push(get_random_char(&mut rng));
            let pos = (start_y + i as i16);
            positions.push(if pos < 0 { 0 } else { pos as u16 }); // 确保非负再转换
        }
        
        Self {
            x,
            characters,
            positions,
            speed,
            counter: 0,
        }
    }
    
    fn update(&mut self, height: u16, rng: &mut impl Rng) {
        self.counter += 1;
        if self.counter >= self.speed {
            self.counter = 0;
            
            // 更新所有字符的位置
            for pos in &mut self.positions {
                *pos += 1;
                
                // 如果字符已经离开屏幕，随机重置
                if *pos > height + 10 {
                    *pos = rng.gen_range(0..5) as u16 - 10; // 从屏幕上方重新开始
                }
            }
            
            // 随机更新一些字符
            for c in &mut self.characters {
                if rng.gen_bool(0.1) { // 10%的概率更新字符
                    *c = get_random_char(rng);
                }
            }
        }
    }
    
    fn get_char(&self, y: u16) -> Option<char> {
        for (pos, c) in self.positions.iter().zip(self.characters.iter()) {
            if *pos == y {
                return Some(*c);
            }
        }
        None
    }
}

// 获取随机字符
fn get_random_char(rng: &mut impl Rng) -> char {
    // 混合ASCII字符和一些Unicode符号
    let chars = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789";
    chars.chars().nth(rng.gen_range(0..chars.len())).unwrap_or('0')
}

// 简单检测终端是否支持ANSI转义序列（Windows 10及以上通常支持）
fn is_ansi_supported() -> bool {
    true
}

// 获取终端大小（简化实现）
fn get_terminal_size() -> Option<(u16, u16)> {
    // 简化版：使用固定大小，实际应用中可以使用终端库获取
    Some((80, 25))
}

// 检查是否需要退出（简化实现）
fn check_for_exit() -> bool {
    // 注意：在实际应用中，应该使用信号处理来捕获Ctrl+C
    false
}