// Crates


// Modules
mod model;

// Imports
// use std::{ fs::{File}, io::Read };
use std::fs::remove_file;
use model::http_manager;
use std::time::Instant;
use model::fs_manager;
use json::JsonValue;


fn main() {
    // Start Timer
    let init = Instant::now();

    // Send A Web Request To Package Name
    let response = http_manager::send_package_request("@nastyox/rando.js");
    
    // TODO: Replace ["2.0.0"] With "version"
    // TODO: Put this in Package Class
    
    let mut package_name = response["name"].to_string();
    if package_name.contains('/') {
        let arr = package_name.split("/");
        package_name = arr.last().unwrap().to_string();
    }

    let url: &JsonValue = &response["versions"]["2.0.0"]["dist"]["tarball"];
    http_manager::download(url.to_string().as_str(), format!("{}.tar.gz", package_name).as_str());
    
    match fs_manager::decompress(&package_name, format!(r"node_modules\{}.tar.gz", package_name).as_str()) {
        Ok(_) => {},
        Err(e) => eprintln!("Failed To Decompress Tarball: {}", e),
    }

    match remove_file(format!(r"node_modules\{}.tar.gz", package_name).as_str()) {
        Ok(_) => println!(""),
        Err(e) => eprintln!("Failed To Delete Tarball: {}", e), 
    }

    // End Timer
    let end = Instant::now();
    println!("\nExecution Completed With Exit Code 0 in {:.2}s", (end - init).as_secs_f32());
}


// fn parse_json_file(file_path: &str) -> JsonValue {
//     /*
//         Open a file and return the data as `JsonValue`
//     */

//     // Open File
//     let mut file = File::open(file_path).expect("Can't Open File!");
    
//     // Initialise Mutable Instance To Write Contents Into
//     let mut contents = String::new();
   
//     // Write the contents of the file path into contents
//     file.read_to_string(&mut contents).expect("Failed To Read File Contents!");
    
//     // Parse JSON And Convert to JsonValue
//     let res = parse(&contents).unwrap_or_else(| error | {
//         // Display Error Message And Exit
//         eprintln!("An Error Occured While Parsing \"{}\" {}", file_path, error);
//         process::exit(1);
//     });

//     // Return JsonValue Data
//     res
// }
