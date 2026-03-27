use anyhow::Result;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

pub fn perform_restore(backup_zip_path: &Path, xampp_path: &Path) -> Result<()> {
    let file = File::open(backup_zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
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
