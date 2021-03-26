use json::{ parse, JsonValue };
use std::process;
use reqwest;

pub fn send_package_request(name: &str) -> JsonValue {
    let mut data = String::new();

    match reqwest::blocking::get(format!("https://registry.npmjs.org/{}", name)) {
        Ok(response) => {
            // Check if 200 OK
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => {
                        data = text;
                    },
                    Err(err) => eprintln!("Could Not Read Response JSON, {}", err)
                }
            } else {
                println!("Response Was Not 200 OK");
            }
        }
        Err(err) => println!("Failed To Send Request: {}", err)
    }

    // Parse JSON And Convert to JsonValue
    let res = parse(&data).unwrap_or_else(| error | {
        // Display Error Message And Exit
        eprintln!("An Error Occured While Parsing The Json Data : {}", error);
        process::exit(1);
    });

    // Return JsonValue Response
    res
}