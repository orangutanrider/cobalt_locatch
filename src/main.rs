use serde::*;
use reqwest::*;
use reqwest::header::*;
use task::spawn_blocking;
use tokio::*;

fn main() {
    let async_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    async_runtime.block_on(ping_test());
}

async fn ping_test() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}