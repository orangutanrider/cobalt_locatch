//! File intake as of cli input

// Logging todo
// probably replace the prints with logging

use locatch_macro::*;
use crate::cli::*;
use crate::serial_input::{Config, FilenameMacro, List, TicketMacro};

use std::{fs, path::PathBuf};

/// The output of reception
/// Deserialized files, from the CLI input.
pub type RecievedInput = (Config, List);

#[inline]
pub fn reception(cli: &Cli) -> Result<RecievedInput, ()> {
    // Recieve inputs
    let config = match config_reception(&cli.config) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let mut list = match list_reception(&cli.list) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let filename_macro = match filename_macro_reception(&cli.filename_macro) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    let ticket_macro = match ticket_macro_reception(&cli.ticket_macro) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    // Apply macros
    list.apply_internal_macros();
    match filename_macro {
        Some(filename_macro) => list.apply_filename_macro(&filename_macro),
        None => {/* Do Nothing */},
    }
    match ticket_macro {
        Some(ticket_macro) => list.apply_ticket_macro(&ticket_macro),
        None => {/* Do Nothing */},
    }

    // Return
    return Ok((config, list))
}

/// Gets config, and deserializes it.
/// Config has a fallback path associated with it, if it was not inputted via cli, then the system will try to get it via the default path.
#[inline]
fn config_reception(cli: &Option<PathBuf>) -> Result<Config, ()> {
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
fn list_reception(cli: &PathBuf) -> Result<List, ()> {
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
fn filename_macro_reception(cli: &Option<PathBuf>) -> Result<Option<FilenameMacro>, ()> {
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
fn ticket_macro_reception(cli: &Option<PathBuf>) -> Result<Option<TicketMacro>, ()> {
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