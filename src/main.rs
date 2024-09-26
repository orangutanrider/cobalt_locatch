use serde::*;
use reqwest::*;
use reqwest::header::*;
use serde_json::json;
use task::spawn_blocking;
use tokio::*;

mod cobalt;


fn main() {
    let async_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    //async_runtime.block_on(ping_test());
    async_runtime.block_on(cobalt_test());
}

async fn cobalt_test() -> Result<()> { 
    let client = reqwest::Client::new();

    let response = client.post("http://localhost:9000")
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(json!({
            "url": "https://www.youtube.com/watch?v=yQM1KM66QT4",
            "downloadMode": "audio"
        }).to_string())
        .send()
        .await?;

    response.text();

    Ok(())
}

// async fn ping_test() -> Result<()> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
// 
//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     Ok(())
// }