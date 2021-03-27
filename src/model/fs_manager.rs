use std::fs::{File};
use flate2::read::GzDecoder;
use tar::Archive;

pub fn decompress(package_name: &str, path: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(format!(r"C:\Users\xtrem\Desktop\volt\node_modules\{}", package_name))?;
    
    Ok(())
}
