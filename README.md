# catshell 🐱

一个用Rust编写的简单而功能丰富的Shell命令实现，支持多种常用Unix/Linux风格命令，带有emoji增强的用户体验。

## 🚀 已实现的功能

### 1. 文件和目录操作

- **ls 命令**: 列出目录内容
  - 显示隐藏文件 (`-a` 或 `--all`)
  - 长格式显示 (`-l`)
  - 递归列出子目录 (`-r` 或 `--recursive`)
  - 带有可爱的emoji文件类型标识 📁 📄 🔗 ❓

- **cd 命令**: 更改当前工作目录
  - 支持相对路径和绝对路径
  - 支持波浪号 `~` 解析为主目录
  - 目录切换成功后显示新目录和定位emoji 📍

- **pwd 命令**: 显示当前工作目录

- **mkdir 命令**: 创建新目录
  - 支持同时创建多个目录
  - 递归创建父目录 (`-p` 或 `--parents`)
  - 成功/失败提示和emoji反馈 ✅ ❌

- **rm 命令**: 删除文件或目录
  - 递归删除目录内容 (`-r` 或 `--recursive`)
  - 强制删除，忽略不存在的文件 (`-f` 或 `--force`)

- **cat 命令**: 查看文件内容
  - 支持查看多个文件
  - 显示所有行号 (`-n`)
  - 只显示非空行的行号 (`-b`)
  - 处理文本和二进制文件

### 2. 系统和网络命令

- **uname 命令**: 显示系统信息
  - 模拟 `uname -a` 的输出格式
  - 包含主机名、操作系统版本、内核信息等
  - 带有计算机emoji 🖥️

- **curl 命令**: 发起HTTP/HTTPS请求
  - 支持GET、POST、PUT、DELETE等HTTP方法
  - 显示响应头 (`-i`)
  - 添加自定义请求头 (`-H`)
  - 发送请求数据 (`-d`)
  - 保存响应到文件 (`-o`)
  - 只显示状态码 (`--head`)
  - 静默模式 (`-s`)

## 📋 要求

- Rust 1.70+ 和 Cargo
- Windows 操作系统（当前实现针对Windows优化）

## 🛠️ 安装

1. 克隆仓库
   ```bash
   git clone https://github.com/yourusername/catshell.git
   cd catshell
   ```

2. 构建项目
   ```bash
   cargo build --release
   ```

3. 可执行文件位置
   - Windows: `target/release/catshell.exe`

## 📖 使用方法

### 基本用法

使用以下命令格式运行catshell：
```bash
cargo run -- <command> [options] [arguments]
```

或者先构建，然后直接运行可执行文件：
```bash
./target/debug/catshell <command> [options] [arguments]
```

### 命令示例

#### 文件和目录操作

```bash
# 列出当前目录内容
cargo run -- ls

# 列出当前目录内容（包括隐藏文件）
cargo run -- ls -a

# 长格式列出当前目录
cargo run -- ls -l

# 更改目录到src
cargo run -- cd src

# 显示当前工作目录
cargo run -- pwd

# 创建新目录
cargo run -- mkdir new_folder

# 创建嵌套目录
cargo run -- mkdir -p parent/child/grandchild

# 删除文件
cargo run -- rm file.txt

# 递归删除目录
cargo run -- rm -r folder

# 查看文件内容
cargo run -- cat README.md

# 查看文件内容并显示行号
cargo run -- cat -n file.txt
```

#### 系统和网络命令

```bash
# 显示系统信息
cargo run -- uname

# 发起GET请求
cargo run -- curl https://example.com

# 发起POST请求
cargo run -- curl -X POST -d "hello=world" https://httpbin.org/post

# 显示响应头
cargo run -- curl -i https://example.com

# 添加自定义请求头
cargo run -- curl -H "User-Agent: catshell-curl" https://httpbin.org/headers

# 保存响应到文件
cargo run -- curl -o output.html https://example.com

# 只获取HTTP状态码
cargo run -- curl --head https://example.com
```

## 🎯 设计理念

catshell项目旨在学习Rust编程语言和系统编程概念，同时提供一个简单但功能齐全的命令行工具集合。主要特点：

- **简单易用**: 遵循常见Shell命令的使用习惯，降低学习成本
- **视觉增强**: 使用emoji提供直观的视觉反馈
- **跨平台思考**: 核心功能考虑跨平台兼容性，当前实现针对Windows优化
- **学习资源**: 代码结构清晰，适合Rust初学者学习参考

## 🤝 贡献

欢迎提出问题和改进建议！如果你有兴趣为项目贡献代码，请提交Pull Request。

## 📝 许可证

[MIT License](LICENSE)

## 💡 鸣谢

这个项目受到了Unix/Linux命令行工具的启发，使用了Rust语言的强大功能来实现。特别感谢Rust社区提供的优秀工具链和库。