//! File input reception

use locatch_macro::*;
use crate::{SerialConfig, SerialInput};

use std::{fs, path::PathBuf};

/// Gets config, and deserializes it
#[inline]
pub fn config_reception(cli: &Option<PathBuf>) -> Result<SerialConfig, ()> {
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
            const DEFAULT_CONFIG_PATH: &str = "locatch_config.json";
            match fs::read_to_string(DEFAULT_CONFIG_PATH) {
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
    let config = match SerialConfig::from_json(&config) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of config file: {}", err);
            return Err(())
        },
    };

    return Ok(config)
}

/// Gets input, deserializes, and applies the macro
#[inline]
pub fn input_reception(cli: &PathBuf) -> Result<SerialInput, ()> {
    // Recieve input
    let input = match fs::read_to_string(cli) {
        Ok(ok) => {
            println!("Input file recieved");
            ok
        },
        Err(err) => {
            println!("Error with input file: {}", err);
            return Err(())
        },
    };

    // Deserialize input
    let mut input = match SerialInput::from_json(&input) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of input file: {}", err);
            return Err(())
        },
    };

    // apply macro
    input.apply_macro();

    return Ok(input)
}