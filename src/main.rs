#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod extract;
mod inject;
mod options;
mod settings;
mod storefile;
mod walker;

use settings::Settings;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = options::Options::new(Some(args.as_ref()));
    let settings = Settings::load_settings("./.docyconf.json");
    let targets = walker::walk(&settings);
    let store_data = match storefile::StoreFile::load() {
        Ok(value) => value,
        Err(_) => storefile::StoreFile::new(),
    };

    match args[1].as_str() {
        "inject" | "in" => inject::run(store_data, options),
        "extract" | "ex" => extract::run(targets, store_data, options),
        _ => {
            println!("Please provide extra argument:\n- inject\n- extract")
        }
    };
}
