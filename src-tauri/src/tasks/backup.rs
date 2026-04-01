use anyhow::Result;
use chrono::Local;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn perform_backup<F>(xampp_path: &Path, backup_dest: &Path, on_progress: F) -> Result<PathBuf> 
where F: Fn(f32) {
    let date_str = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_file_name = format!("xampp_backup_{}.zip", date_str);
    
    if !backup_dest.exists() {
        std::fs::create_dir_all(backup_dest)?;
    }
    
    let backup_path = backup_dest.join(&backup_file_name);
    let file = File::create(&backup_path)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let include_paths = vec![
        "htdocs",
        "apache\\conf",
        "mysql\\data",
        "mysql\\bin\\my.ini",
        "php\\php.ini",
    ];

    let total = include_paths.len() as f32;

    for (i, path) in include_paths.iter().enumerate() {
        on_progress(i as f32 / total);
        let full_path = xampp_path.join(path);
        
        if !full_path.exists() {
            continue;
        }

        if full_path.is_dir() {
            for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
                let path_in_zip = match entry.path().strip_prefix(xampp_path) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                
                let path_str = path_in_zip.to_string_lossy().replace("\\", "/");
                if path_str.is_empty() { continue; }

                if entry.file_type().is_file() {
                    if let Ok(mut f) = File::open(entry.path()) {
                        let _ = zip.start_file(&path_str, options);
                        let _ = std::io::copy(&mut f, &mut zip);
                    }
                } else if entry.file_type().is_dir() {
                    let _ = zip.add_directory(&path_str, options);
                }
            }
        } else {
            let path_in_zip = match full_path.strip_prefix(xampp_path) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let path_str = path_in_zip.to_string_lossy().replace("\\", "/");
            
            if let Ok(mut f) = File::open(&full_path) {
                let _ = zip.start_file(&path_str, options);
                let _ = std::io::copy(&mut f, &mut zip);
            }
        }
    }

    on_progress(1.0);
    zip.finish()?;
    Ok(backup_path)
}
