
use std::future::Future;

use serde::*;
use reqwest::*;
use reqwest::header::*;
use serde_json::json;
use task::spawn_blocking;
use tokio::*;

mod input;
mod cobalt;
use cobalt::*;

use std::result::Result as StdResult;
use reqwest::Result as ReqResult;


fn main() {
    let async_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    //async_runtime.block_on(ping_test());
    //async_runtime.block_on(cobalt_test());
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

async fn post_cobalt(client: &Client, post: &str, body: &'static str) -> impl Future<Output = ReqResult<Response>> { 
    return client.post(post)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();
}