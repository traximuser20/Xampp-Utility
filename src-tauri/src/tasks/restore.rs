use anyhow::Result;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

pub fn perform_restore<F>(backup_zip_path: &Path, xampp_path: &Path, on_progress: F) -> Result<()> 
where F: Fn(f32) {
    let file = File::open(backup_zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let total = archive.len();

    for i in 0..total {
        on_progress(i as f32 / total as f32);
        let mut file = archive.by_index(i)?;
        let out_path = xampp_path.join(file.name().replace("/", "\\"));

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&out_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
