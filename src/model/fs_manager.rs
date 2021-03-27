use flate2::read::GzDecoder;
use std::path::Path;
use std::fs::{File, create_dir};
use tar::Archive;


pub fn init() {
    if Path::new("node_modules").is_dir() == false {
        match create_dir("node_modules")  {
            Ok(_) => {},
            Err(e) => eprintln!("An Error Occured While Creating node_modules: {}", e),
        }
    }

    if Path::new(r"C:\Users\prana\.volt").is_dir() == false {
        match create_dir(r"C:\Users\prana\.volt")  {
            Ok(_) => {},
            Err(e) => eprintln!("An Error Occured While Creating node_modules: {}", e),
        }
    }
}

pub fn decompress(developer_name: &str, package_name: &str, path: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(format!(r"D:\prana\Programming\My Projects\volt\node_modules\{}\{}", developer_name, package_name))?;
    
    Ok(())
}


// pub fn cache(name: &str, file_path: &str) {
//     let cache_path = r"C:\Users\prana\.volt";
//     copy(file_path, format!("{}{}.tar.gz", cache_path, name));
// }