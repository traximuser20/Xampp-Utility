use std::path::PathBuf;
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
