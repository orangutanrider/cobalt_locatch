use lib::*;

use std::{
    fs,
    path::PathBuf
};

use clap::*;

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

    println!("Attempting to connect to cobalt instance");
    println!("@ {}", &config.cobalt_url);
    match async_runtime.block_on(get_cobalt(&config.cobalt_url)) {
        Ok(_) => {/* Do nothing */},
        Err(err) => {
            exit_msg!("{}", err); return;
        },
    }
}

async fn make_requests(cobalt_url: &str, input: &SerialInput) {

}

async fn get_cobalt(cobalt_url: &str) -> Result<(), reqwest::Error> {
    let response = match reqwest::get(cobalt_url).await {
        Ok(ok) => {
            println!("Succesfully connected to cobalt");
            ok
        },
        Err(err) => {
            println!("Couldn't connect to cobalt");
            return Err(err);
        },
    };

    let response = match response.text().await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error: {}", err);
            println!("Couldn't get the response text from cobalt");
            println!("Since cobalt was succesfully connected to, will try to continue execution anyways");
            return Ok(())
        },
    };

    let response = match GetResponse::from_json(&response) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error: {}", err);
            println!("Failed to deserialize cobalt response");
            println!("This could indicate that an incompatible version of cobalt is being connected to");
            println!("Will try to continue execution anyways");
            return Ok(())
        },
    };

    println!("Cobalt version {} @commit {}", response.cobalt.version, response.git.commit);
    Ok(())
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