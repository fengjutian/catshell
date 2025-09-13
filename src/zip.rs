use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

/// 创建zip压缩文件
pub fn create_zip(entries: &[String], output_path: &str) {
    let file = match File::create(output_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("无法创建压缩文件 {}: {}", output_path, err);
            return;
        }
    };

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in entries {
        let path = Path::new(entry);
        if !path.exists() {
            eprintln!("警告: {} 不存在，跳过", entry);
            continue;
        }

        if path.is_file() {
            add_file_to_zip(&mut zip, path, path.file_name().unwrap().to_str().unwrap(), options).unwrap_or_else(|err| {
                eprintln!("添加文件 {} 失败: {}", entry, err);
            });
        } else if path.is_dir() {
            add_directory_to_zip(&mut zip, path, "", options).unwrap_or_else(|err| {
                eprintln!("添加目录 {} 失败: {}", entry, err);
            });
        }
    }

    if let Err(err) = zip.finish() {
        eprintln!("完成压缩文件失败: {}", err);
        return;
    }

    println!("成功创建压缩文件: {}", output_path);
}

/// 解压缩zip文件
pub fn extract_zip(zip_path: &str, output_dir: &str) {
    let file = match File::open(zip_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("无法打开压缩文件 {}: {}", zip_path, err);
            return;
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            eprintln!("无法解析压缩文件 {}: {}", zip_path, err);
            return;
        }
    };

    // 确保输出目录存在
    if let Err(err) = fs::create_dir_all(output_dir) {
        eprintln!("无法创建输出目录 {}: {}", output_dir, err);
        return;
    }

    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("读取压缩文件条目失败: {}", err);
                continue;
            }
        };

        let output_path = Path::new(output_dir).join(file.name());

        if file.name().ends_with('/') {
            // 是目录，创建目录
            if let Err(err) = fs::create_dir_all(&output_path) {
                eprintln!("创建目录 {} 失败: {}", output_path.display(), err);
                continue;
            }
        } else {
            // 是文件，创建父目录并写入文件
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    if let Err(err) = fs::create_dir_all(parent) {
                        eprintln!("创建父目录 {} 失败: {}", parent.display(), err);
                        continue;
                    }
                }
            }

            let mut target_file = match File::create(&output_path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("创建文件 {} 失败: {}", output_path.display(), err);
                    continue;
                }
            };

            if let Err(err) = io::copy(&mut file, &mut target_file) {
                eprintln!("写入文件 {} 失败: {}", output_path.display(), err);
                continue;
            }
        }
    }

    println!("成功解压缩到: {}", output_dir);
}

// 将单个文件添加到zip中
fn add_file_to_zip(zip: &mut ZipWriter<File>, path: &Path, name: &str, options: FileOptions) -> zip::result::ZipResult<()> {
    let mut file = File::open(path)?;
    zip.start_file(name, options)?;
    io::copy(&mut file, zip)?;
    Ok(())
}

// 将目录添加到zip中
fn add_directory_to_zip(zip: &mut ZipWriter<File>, path: &Path, prefix: &str, options: FileOptions) -> zip::result::ZipResult<()> {
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy();
        let zip_path = if prefix.is_empty() {
            entry_name.to_string()
        } else {
            format!("{}/{}", prefix, entry_name)
        };

        if entry_path.is_file() {
            add_file_to_zip(zip, &entry_path, &zip_path, options)?;
        } else if entry_path.is_dir() {
            zip.start_file(format!("{}/", zip_path), options)?;
            add_directory_to_zip(zip, &entry_path, &zip_path, options)?;
        }
    }

    Ok(())
}