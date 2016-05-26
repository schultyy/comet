mod config;
mod builder;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use std::io::Error;


fn read_config_file() -> Result<String, Error> {
    let mut f = try!(File::open(".comet.json"));
    let mut file_content = String::new();
    try!(f.read_to_string(&mut file_content));
    Ok(file_content)
}

fn main() {
    println!("comet ☄");
    let json = match read_config_file() {
        Ok(fc) => fc,
        Err(err) => {
            println!("[ERR] Could not read configuration file");
            println!("[ERR] Reason {:?}", err);
            std::process::exit(1)
        }
    };

    let configuration = match config::from_json(&json) {
        Ok(cfg) => cfg,
        Err(err) => {
            println!("[ERR] Failed to parse configuration file");
            println!("[ERR] Reason: {:?}", err);
            std::process::exit(1)
        }
    };

    println!("configuration language {}", configuration.language);
    println!("configuration script {:?}", configuration.script);

    match builder::build(configuration) {
        Ok(results) => {
            println!("Was success {}", results.was_success);
        },
        Err(err) => {
            println!("[ERR] {:?}", err);
        }
    }
}
