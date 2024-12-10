mod download; use download::*;
mod req; use req::*;
mod sanitize; use sanitize::*;

use lib::*;

use std::{
    fs, 
    future::Future, 
    path::PathBuf,
};

use clap::*;
use reqwest::{Client, Response};

macro_rules! exit_msg {($($tt:tt)*) => {
    print!("Exiting (");
    print!($($tt)*);
    println!(")")
};}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>, 

    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,
}

const DEFAULT_CONFIG_PATH: &str = "cobalt_config.json";

fn main() {    
    let cli = Cli::parse();

    println!("{:?}", cli.input);

    // Recieve config
    let config = match cli.config {
        Some(config) => {
            match fs::read_to_string(config) {
                Ok(ok) => {
                    println!("Config file recieved");
                    ok
                },
                Err(err) => {
                    println!("Error with config file");
                    exit_msg!("{}", err); return;
                },
            }
        },
        None => {
            match fs::read_to_string(DEFAULT_CONFIG_PATH) {
                Ok(ok) => {
                    println!("Config file recieved");
                    ok
                },
                Err(err) => {
                    println!("Error with config file");
                    exit_msg!("{}", err); return;
                },
            }
        },
    };

    // Deserialize config
    let config = match SerialConfig::from_json(&config) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of config file");
            exit_msg!("{}", err); return;
        },
    };

    // Recieve input
    let input = match fs::read_to_string(&cli.input) {
        Ok(ok) => {
            println!("Input file recieved");
            ok
        },
        Err(err) => {
            println!("Error with input file");
            exit_msg!("{}", err); return;
        },
    };

    // Deserialize input
    let mut input = match SerialInput::from_json(&input) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error with deserialization of input file");
            exit_msg!("{}", err); return;
        },
    };

	// apply macro
    input.apply_macro();

    // start tokio
    let async_runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build() 
    {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to start tokio async runtime");
            exit_msg!("{}", err); return;
        },
    };

	// start web client
    println!("Starting web client...");
    let client = Client::new();

	// get
    println!("Attempting to connect to cobalt instance");
    println!("@ {}", &config.cobalt_url);
    match async_runtime.block_on(get_cobalt(&client, &config.cobalt_url)) {
        Ok(_) => {/* Do nothing */},
        Err(err) => {
            exit_msg!("{}", err); return;
        },
    }

	// post
    println!("Making requests");
    let len = input.requests.len();
    let responses = make_requests(&client, &config.cobalt_url, &input, len);

	// await responses
    println!("Waiting for responses...");
    let responses = async_runtime.block_on(unwrap_responses(responses, len));
    let responses = request_response_texts(responses, len);
    let responses = async_runtime.block_on(unwrap_pending_texts(responses, len));
    
	// deserialize responses
    println!("Responses recieved"); //log?
    println!("Deserializing responses"); //log?
    let responses = deserialize_responses(responses, len);
    
    // Technically, you only have to allocate enough space for a single vec with capacity "len", sized by whichever data structure is the largest out of the union types.
    // Theoretically, you would create a new vector/allocation, sort the data into it (so that they are in homogenous blocks), and then create slices for each type.
    // Hypothetically, more computation in creating the storage type, but more memory efficient afterwards.

	// Filter responses
    let mut errors = Vec::with_capacity(len);
    let mut pickers = Vec::with_capacity(len);
    let mut tunnels = Vec::with_capacity(len);
    filter_responses(responses.into_iter(), &mut errors, &mut pickers, &mut tunnels);

    // Sanitize file names
    let tunnels_future = tunnels_sanitize(&mut tunnels);
    let picker_future = pickers_sanitize(&mut pickers);
    async_runtime.block_on(tunnels_future);
    async_runtime.block_on(picker_future);

	// Start downloads
    let tunnel_downloads = start_download_tunnels(&client, tunnels.iter(), len);

    // await downloads
}

// todo!
// File output to output directory input

fn handle_errors() {
    todo!()
}

// Pseudocode
/*
    Read file into a Vec
    
    Async SIMD Vec into cobalt API requests (New Vec)
        Wait...
    Async SIMD Vec into json (New Vec)
        Wait...
    SIMD deserialize Vec (New Vecs)
    Async SIMD download Vec "Picker"
    Async SIMD download Vec "Redirect"
    Async SIMD download Vec "Tunnel"
        Wait...
    Log Vec "Error"
    
    Log results
    END
*/