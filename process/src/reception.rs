//! File input reception

// Logging todo
// probably replace the prints with logging

use locatch_macro::*;
use crate::cli::CONFIG_PATH;
use crate::serial_input::{Config, FilenameMacro, List, TicketMacro};

use std::{fs, path::PathBuf};

/// Gets config, and deserializes it.
/// Config has a fallback path associated with it, if it was not inputted via cli, then the system will try to get it via the default path.
#[inline]
pub fn config_reception(cli: &Option<PathBuf>) -> Result<Config, ()> {
    // Recieve config
    let config = match cli {
        // Path was inputted via cli
        Some(config) => {
            match fs::read_to_string(config) {
                Ok(ok) => {
                    println!("Config file recieved");
                    ok
                },
                Err(err) => {
                    println!("Error with config file: {}", err);
                    return Err(())
                },
            }
        },
        // No path was inputted
        None => {
            match fs::read_to_string(CONFIG_PATH) {
                Ok(ok) => {
                    println!("Config file recieved");
                    ok
                },
                Err(err) => {
                    println!("Error with config file: {}", err);
                    return Err(())
                },
            }
        },
    };

    // Deserialize config
    let config = match Config::from_json(&config) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of config file: {}", err);
            return Err(())
        },
    };

    return Ok(config)
}

/// Gets list, and deserializes it
#[inline]
pub fn list_reception(cli: &PathBuf) -> Result<List, ()> {
    // Recieve list
    let list = match fs::read_to_string(cli) {
        Ok(ok) => {
            println!("List file recieved");
            ok
        },
        Err(err) => {
            println!("Error with list file: {}", err);
            return Err(())
        },
    };

    // Deserialize list
    let list = match List::from_json(&list) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of list file: {}", err);
            return Err(())
        },
    };

    return Ok(list)
}

#[inline]
pub fn filename_macro_reception(cli: &Option<PathBuf>) -> Result<Option<FilenameMacro>, ()> {
    let cli = match cli {
        Some(val) => val,
        None => {
            println!("No filename macro input, outside of list");
            return Ok(None)
        },
    };

    // Recieve file
    let serial = match fs::read_to_string(cli) {
        Ok(ok) => {
            println!("Filename macro file recieved");
            ok
        },
        Err(err) => {
            println!("Error with filename macro: {}", err);
            return Err(())
        },
    };

    // Deserialize file
    let deserial = match FilenameMacro::from_json(&serial) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of filename macro: {}", err);
            return Err(())
        },
    };

    return Ok(Some(deserial))
}

#[inline]
pub fn ticket_macro_reception(cli: &Option<PathBuf>) -> Result<Option<TicketMacro>, ()> {
    let cli = match cli {
        Some(val) => val,
        None => {
            println!("No ticket macro input, outside of list");
            return Ok(None)
        },
    };

    // Recieve file
    let serial = match fs::read_to_string(cli) {
        Ok(ok) => {
            println!("Ticket macro file recieved");
            ok
        },
        Err(err) => {
            println!("Error with ticket macro file: {}", err);
            return Err(())
        },
    };

    // Deserialize file
    let deserial = match TicketMacro::from_json(&serial) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of ticket macro: {}", err);
            return Err(())
        },
    };

    return Ok(Some(deserial))
}