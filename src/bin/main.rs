fn main() {

}

/* 
use std::future::Future;
use std::path::PathBuf;
use std::env;
use std::fs;

use input::SerialInput;
use serde::*;
use reqwest::*;
use reqwest::header::*;
use serde_json::json;
use task::spawn_blocking;
use tokio::*;
use clap::Parser;

mod input;
mod cobalt;
use cobalt::*;

#[cfg(feature = "api-test")]
mod api_test;

use serde_json::Error as JsonError;
use std::result::Result as StdResult;
use reqwest::Result as ReqResult;
use reqwest::Error as ReqError;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let input_path = cli.input;
    println!("{:?}", input_path);

    let input = match fs::read_to_string(input_path) {
        Ok(ok) => {
            println!("File recieved");
            ok
        },
        Err(err) => {
            print!("Error with recieving input file: ");
            println!("{}", err);
            println!("Exiting due to error");
            return;
        },
    };

    let input = match serde_json::de::from_str::<SerialInput>(&input) {
        Ok(ok) => {
            println!("Deserialization succesful");
            ok
        },
        Err(err) => {
            print!("Error with deserialization of input: ");
            println!("{}", err);
            println!("Exiting due to error");
            return;
        },
    };
    
    let async_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    //async_runtime.block_on(ping_test());
    //async_runtime.block_on(cobalt_test());
}

fn take_input() {

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

// async fn post_cobalt(body: String) { 
//     let client = reqwest::Client::new();
// 
//     let response = client.post("http://localhost:9000")
//         .header(ACCEPT, "application/json")
//         .header(CONTENT_TYPE, "application/json")
//         .body(json!({
//             "url": "https://www.youtube.com/watch?v=yQM1KM66QT4",
//             "downloadMode": "audio"
//         }).to_string())
//         .send()
//         .await?;
// 
//     Ok(response)
// }

// fn to_code(json: &str) -> StdResult<CobaltResponse, ()> { ... }
// into_response(json)

// async fn to_json(response: Response) -> impl Future<Output = ReqResult<String>> { ... }
// response.text()

#[inline]
pub(crate) async fn post_cobalt(client: &Client, url: &str, body: &'static str) -> impl Future<Output = ReqResult<Response>> { 
    return client.post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();
}
*/