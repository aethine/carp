use crate::cratesio;
use crate::utils;

use std::io::{Error, ErrorKind};

pub fn help() {
    println!(
        "carp: A basic CLI Cargo.toml editor
    - carp help: Prints this message
    - carp list: Lists all dependencies
    - carp add <crate name> [crate version]: Adds a dependency
    - carp rem <crate name>: Removes a dependency
    - carp change <crate name> <crate version>: Changes a dependency's version
    - carp check [crate name]: Checks if a dependency is up-to-date, or all dependencies if none are specified
    - carp update [crate name]: Updates a dependency, or all dependencies if none are specified"
    )
}
pub fn list() {
    match utils::read_parse_dependencies(&utils::get_toml_path()) {
        Ok(v) => {
            for (k, v) in v {
                println!("{} ({})", k, v)
            }
        }
        Err(e) => println!("{}", e),
    }
}
pub fn add(name: &str, version: Option<&str>) -> Result<(), Error> {
    let latest = match cratesio::crate_latest(name) {
        Ok(v) => v,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("{:?}", e))),
    };
    let ver = match version {
        Some(v) => v,
        None => &latest,
    };

    let path = &utils::get_toml_path();
    let mut dependencies = utils::read_parse_dependencies(path)?;
    dependencies.insert(String::from(name), String::from(ver));
    return utils::write_dependencies(path, &dependencies);
}
