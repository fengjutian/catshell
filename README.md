# catshell

A simple shell command implementation in Rust, starting with the `ls` and `pwd` commands.

## 🚀 Features

- **ls command**: List directory contents with various options
  - Display hidden files (`-a` or `--all`)
  - Long format listing (`-l`)
  - Recursive directory listing (`-r` or `--recursive`)
- **pwd command**: Display the current working directory
- Written in Rust for performance and reliability
- Cross-platform compatibility

## 📋 Requirements

- Rust 1.70+ and Cargo

## 🛠️ Installation

1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/catshell.git
   cd catshell
   ```

2. Build the project
   ```bash
   cargo build --release
   ```

3. The executable will be available in `target/release/catshell.exe` (Windows) or `target/release/catshell` (Unix-like systems)

## 📖 Usage

### Basic Usage

List files in the current directory:
```bash
cargo run
```

List files in a specific directory:
```bash
cargo run -- /path/to/directory
```

### Command Line Options

cargo run -- -l  # 长格式显示当前目录
cargo run -- -a  # 显示所有文件（包括隐藏文件）

- `-a`, `--all`: Show hidden files
- `-l`: Use a long listing format
- `-r`, `--recursive`: Recursively list subdirectories