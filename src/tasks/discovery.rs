use std::path::{Path, PathBuf};
use std::fs;

pub fn discover_xampp() -> Option<PathBuf> {
    // Try to find xampp in standard locations
    let common_locations = [
        "C:\\xampp",
        "D:\\xampp",
        "E:\\xampp",
    ];

    for loc in common_locations {
        let path = PathBuf::from(loc);
        if path.join("php").join("php.exe").exists() {
            return Some(path);
        }
    }

    // Try to find by scanning drives (limited to root for performance)
    if let Ok(entries) = fs::read_dir("C:\\") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().map_or(false, |n| n == "xampp") {
                if path.join("php").join("php.exe").exists() {
                    return Some(path);
                }
            }
        }
    }

    None
}

pub fn get_xampp_version(xampp_path: &Path) -> Option<String> {
    let php_exe = xampp_path.join("php").join("php.exe");
    if php_exe.exists() {
        if let Ok(output) = std::process::Command::new(php_exe).arg("-v").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                if let Some(pos) = line.find("PHP ") {
                    let version_part = &line[pos + 4..];
                    if let Some(space_pos) = version_part.find(' ') {
                        return Some(version_part[..space_pos].to_string());
                    }
                }
            }
        }
    }
    None
}
