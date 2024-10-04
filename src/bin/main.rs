use lib::*;

use core::slice;
use std::{
    fs, 
    future::Future, 
    path::PathBuf,
};

use clap::*;
use reqwest::{Client, Response};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

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

    let client = Client::new();
    println!("Making requests");
    let len = input.requests.len();
    let responses = make_requests(&client, &config.cobalt_url, &input, len);

    println!("Waiting for responses...");
    let responses = async_runtime.block_on(unwrap_responses(responses, len));
    let responses = request_response_texts(responses, len);
    let responses = async_runtime.block_on(unwrap_pending_texts(responses, len));
    
    println!("Responses recieved"); //log?
    println!("Deserializing responses"); //log?
    let responses = deserialize_responses(responses, len);
    
    // Technically, you only have to allocate enough space for a single vec with capacity "len", sized by whichever data structure is the largest out of the union types.
    // Theoretically, you would create a new vector/allocation, sort the data into it (so that they are in homogenous blocks), and then create slices for each type.
    // Hypothetically, more computation in creating the storage type, but more memory efficient afterwards.

    let mut errors = Vec::with_capacity(len);
    let mut pickers = Vec::with_capacity(len);
    let mut tunnels = Vec::with_capacity(len);
    seperate_deserialized(responses.into_iter(), &mut errors, &mut pickers, &mut tunnels);

    // Sanitize file names
    let tun_future = tunnels_sanitize(&mut tunnels);
    let picker_future = pickers_sanitize(&mut pickers);
    async_runtime.block_on(tun_future);
    async_runtime.block_on(picker_future);

    let tunnel_downloads = start_download_tunnels(tunnels.iter(), len);
    // wait downloads
}

// todo!
// File output to output directory input

async fn tunnels_sanitize(tunnels: &mut Vec<TunnelResponse>) {
    for tunnel in tunnels.iter_mut() { // par SIMD possible?
        sanitize_filename::sanitize(&mut tunnel.filename);
    }
}

async fn pickers_sanitize(pickers: &mut Vec<PickerResponse>) {
    for picker in pickers.iter_mut() { // par SIMD possible?
        let Some(audio_filename) = &mut picker.audio_filename else {
            continue;
        };

        sanitize_filename::sanitize(audio_filename);
    }
}

enum DownloadError {
    FileError(std::io::Error),
    ReqwestError(reqwest::Error),
}

fn start_download_tunnels<'a>(
    iter: slice::Iter<'a, TunnelResponse>, 
    len: usize
) -> Vec<impl Future<Output = Result<(), DownloadError>> + use<'a>> {
    let mut futures = Vec::with_capacity(len);
    
    for tunnel in iter { 
        futures.push(download(&tunnel.url, &tunnel.filename));
    }

    return futures;
}

// https://github.com/lostdusty/retrobalt/blob/main/module_downloader.go#L22-L49
// https://discord.com/channels/1049706293700591677/1049740077460377660/1291849923519578154
async fn start_download_pickers<'a>(
    iter: slice::Iter<'a, PickerResponse>, 
    len: usize
) -> Vec<impl Future<Output = Result<(), DownloadError>> + use<'a>> {
    let mut futures = Vec::with_capacity(len);

    let mut outer_index = 0;
    for picker in iter { 
        let mut inner_index = 0;
        for picker_obj in picker.picker.iter() {
            
            inner_index = inner_index + 1;
        }

        let Some(aud_url) = picker.audio else {
            outer_index = outer_index + 1;
            continue;
        };

        let Some(aud_filename) = picker.audio_filename else {
            println!("Recieved an audio url, without recieving its filename"); 
            println!("Logging unimplemented"); //todo!
            //warn!("");

            outer_index = outer_index + 1;
            continue;
        };

        outer_index = outer_index + 1;
    }

    return futures;
}

async fn download(url: &str, filename: &str) -> Result<(), DownloadError> {
    let stream = reqwest::get(url);
    let file = tokio::fs::File::create(filename);

    let mut file = match file.await {
        Ok(ok) => ok,
        Err(err) => return Err(DownloadError::FileError(err)),
    };

    let stream = match stream.await {
        Ok(ok) => ok,
        Err(err) => return Err(DownloadError::ReqwestError(err)),
    };

    let mut stream = stream.bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(ok) => {
                match file.write_all(&ok).await {
                    Ok(_) => {/* Do nothing */},
                    Err(err) => return Err(DownloadError::FileError(err)),
                };
            },
            Err(err) => return Err(DownloadError::ReqwestError(err)),
        }
    }

    match file.flush().await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(DownloadError::FileError(err)),
    }
}


fn handle_errors() {
    todo!()
}

fn deserialize_responses(responses: Vec<String>, len: usize) -> Vec<PostResponse> {
    let mut deserialized = Vec::with_capacity(len);

    for response in responses.iter() { // par SIMD possible?
        match PostResponse::from_json(response) {
            Ok(ok) => deserialized.push(ok),
            Err(err) => {
                println!("Error: {}", err);
                println!("A response could not be deserialized"); 
                println!("Logging unimplemented"); //todo!
                //warn!("");
                continue;
            },
        }
    }

    return deserialized;
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

//type PendingRequest = impl Future<Output = Result<Response, ReqError>>;
macro_rules! PendingRequest {() => {
    impl Future<Output = Result<Response, ReqError>>
};}

fn make_requests(client: &Client, cobalt_url: &str, input: &SerialInput, len: usize) -> Vec<PendingRequest!()> {
    let mut futures = Vec::with_capacity(len);

    for request in input.requests.iter() { // par SIMD possible?
        match request.to_json() {
            Ok(body) => futures.push(post_cobalt(client, cobalt_url, body)),
            Err(err) => {
                println!("Error: {}", err);
                println!("A request could not be serialized"); 
                println!("Logging unimplemented"); //todo!
                //warn!("");
                continue;
            },
        };
    }

    return futures;
}

async fn unwrap_responses(requests: Vec<PendingRequest!()>, len: usize) -> Vec<Response> {
    let mut responses = Vec::with_capacity(len);

    for future in requests.into_iter() { // par SIMD possible?
        match future.await {
            Ok(ok) => responses.push(ok),
            Err(err) => {
                println!("Error: {}", err);
                println!("A response was unable to be recieved"); 
                println!("Logging unimplemented"); //todo!
                //warn!("");
                continue;
            },
        };
    }

    return responses;
}

//type PendingText = impl Future<Output = Result<String, ReqError>>;
macro_rules! PendingText {() => {
    impl Future<Output = Result<String, ReqError>>
};}

fn request_response_texts(responses: Vec<Response>, len: usize) -> Vec<PendingText!()> {
    let mut futures = Vec::with_capacity(len);

    for response in responses.into_iter() { // par SIMD possible?
        futures.push(response.text());
    }

    return futures;
}

async fn unwrap_pending_texts(pending_texts: Vec<PendingText!()>, len: usize) -> Vec<String> {
    let mut texts = Vec::with_capacity(len);

    for text in pending_texts.into_iter() { // par SIMD possible?
        match text.await {
            Ok(ok) => texts.push(ok),
            Err(err) => {
                println!("Error: {}", err);
                println!("Failed to get a response's text"); 
                println!("Logging unimplemented"); //todo!
                //warn!("");
                continue;
            },
        }
    }

    return texts;
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