use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;
use zip::ZipArchive;

pub fn get_xampp_releases() -> BTreeMap<String, String> {
    let mut releases = BTreeMap::new();
    releases.insert("8.2.12".to_string(), "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.2.12/xampp-portable-windows-x64-8.2.12-0-VS16.zip/download".to_string());
    releases.insert("8.1.25".to_string(), "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.1.25/xampp-portable-windows-x64-8.1.25-0-VS16.zip/download".to_string());
    releases.insert("8.0.30".to_string(), "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/8.0.30/xampp-portable-windows-x64-8.0.30-0-VS16.zip/download".to_string());
    releases.insert("7.4.33".to_string(), "https://sourceforge.net/projects/xampp/files/XAMPP%20Windows/7.4.33/xampp-portable-windows-x64-7.4.33-0-VC15.zip/download".to_string());
    releases
}

pub async fn download_xampp<F>(url: &str, dest: &Path, on_progress: F) -> Result<()> 
where F: Fn(f32) {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);
    
    let mut file = tokio::fs::File::create(dest).await?;
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        
        if total_size > 0 {
            on_progress(downloaded as f32 / total_size as f32);
        }
    }

    Ok(())
}

pub fn install_xampp<F>(zip_path: &Path, target_dir: &Path, on_progress: F) -> Result<()> 
where F: Fn(f32) {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let total = archive.len();

    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir)?;
    }

    for i in 0..total {
        on_progress(i as f32 / total as f32);
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();
        
        let out_path = if name.starts_with("xampp/") {
            target_dir.join(name.strip_prefix("xampp/").unwrap().replace("/", "\\"))
        } else {
            target_dir.join(name.replace("/", "\\"))
        };

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

    on_progress(1.0);
    // Run setup_xampp.bat if it exists
    let setup_bat = target_dir.join("setup_xampp.bat");
    if setup_bat.exists() {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "setup_xampp.bat"])
            .current_dir(target_dir)
            .status();
    }

    Ok(())
}
