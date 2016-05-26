mod config;
mod builder;
mod logger;
extern crate rustc_serialize;
extern crate docopt;
extern crate term;

use docopt::Docopt;
use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
comet ☄

Usage:
  comet [options]
  comet --version

Options:
  -h --help              Show this screen.
  --version              Show version.
  -p PATH, --path=PATH   Specifies the working directory. [default: .]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_path: String,
    flag_version: bool
}

fn read_config_file() -> Result<String, Error> {
    let mut f = try!(File::open(".comet.json"));
    let mut file_content = String::new();
    try!(f.read_to_string(&mut file_content));
    Ok(file_content)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
       .and_then(|d| d.decode())
       .unwrap_or_else(|e| e.exit());


    if args.flag_version {
        logger::stdout(format!("comet ☄ -- v{}", VERSION));
        process::exit(0);
    }

    logger::stdout("comet ☄");

    let json = match read_config_file() {
        Ok(fc) => fc,
        Err(err) => {
            logger::stderr("[ERR] Could not read configuration file");
            logger::stderr(format!("[ERR] Reason {:?}", err));
            std::process::exit(1)
        }
    };

    let configuration = match config::from_json(&json) {
        Ok(cfg) => cfg,
        Err(err) => {
            logger::stderr("[ERR] Failed to parse configuration file");
            logger::stderr(format!("[ERR] Reason: {:?}", err));
            std::process::exit(1)
        }
    };

    logger::stdout(format!("configuration language {}", configuration.language));
    logger::stdout(format!("configuration script {:?}", configuration.script));

    let cwd = if args.flag_path.len() == 0 {
        ".".into()
    } else {
        args.flag_path
    };

    match builder::build(configuration, &cwd) {
        Ok(results) => {
            for stats in results.results {
                logger::stdout("------------------\n");
                if stats.success {
                    logger::success(format!("Command: {}", stats.script));
                    logger::success(stats.stdout);
                } else {
                    logger::stderr(format!("Command: {}", stats.script));
                    logger::stderr(stats.stderr);
                }
            }
        },
        Err(err) => {
            println!("[ERR] {:?}", err);
        }
    }
}
