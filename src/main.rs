use std::process;
use std::{ fs::{File}, io::Read };
use json::{ parse, JsonValue };

fn main() {
    // 
    let data: JsonValue = parse_json("express.json");

}


fn parse_json(file_path: &str) -> JsonValue {
    /*
        Open a file and return the data as `JsonValue`
    */

    // Open File
    let mut file = File::open(file_path).expect("Can't Open File!");
    
    // Initialise Mutable Instance To Write Contents Into
    let mut contents = String::new();
   
    // Write the contents of the file path into contents
    file.read_to_string(&mut contents).expect("Failed To Read File Contents!");
    
    // Parse JSON And Convert to JsonValue
    let res = parse(&contents).unwrap_or_else(| error | {
        // Display Error Message And Exit

        eprintln!("An Error Occured While Parsing \"{}\" {}", file_path, error);
        process::exit(1);
    });

    // Return JsonValue Data
    res
}
