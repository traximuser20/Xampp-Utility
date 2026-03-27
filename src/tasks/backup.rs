use anyhow::{Context, Result};
use chrono::Local;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::FileOptions;

pub fn perform_backup(xampp_path: &Path, backup_dest: &Path) -> Result<PathBuf> {
    let date_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_file_name = format!("xampp_backup_{}.zip", date_str);

    if !backup_dest.exists() {
        std::fs::create_dir_all(backup_dest)?;
    }

    let backup_path = backup_dest.join(&backup_file_name);
    let file = File::create(&backup_path)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let include_paths = vec![
        "htdocs",
        "apache\\conf",
        "mysql\\data",
        "mysql\\bin\\my.ini",
        "php\\php.ini",
    ];

    for path in include_paths {
        let full_path = xampp_path.join(path);
        if !full_path.exists() {
            continue;
        }

        if full_path.is_dir() {
            for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
                let path_in_zip = entry.path().strip_prefix(xampp_path)?;
                let path_str = path_in_zip
                    .to_str()
                    .context("invalid path")?
                    .replace("\\", "/");

                if entry.file_type().is_file() {
                    zip.start_file(path_str, options)?;
                    let mut f = File::open(entry.path())?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;
                } else if !path_str.is_empty() {
                    zip.add_directory(path_str, options)?;
                }
            }
        } else {
            let path_in_zip = full_path.strip_prefix(xampp_path)?;
            let path_str = path_in_zip
                .to_str()
                .context("invalid path")?
                .replace("\\", "/");
            zip.start_file(path_str, options)?;
            let mut f = File::open(full_path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(backup_path)
}
