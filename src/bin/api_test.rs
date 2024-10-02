use lib::*;
use reqwest::Client;

const URL: &str = "http://localhost:9000";

fn main() {
    println!("Starting api test");
    println!("Url is {}", URL);

    let async_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    async_runtime.block_on(get_cobalt());

    let client = Client::new();

    async_runtime.block_on(post_entrance_audio(&client));
}

async fn get_cobalt() {
    println!("Attempting to connect, making a GET request");
    let get = match reqwest::get(URL).await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };
    println!("Succesfully connected");

    let get = match get.text().await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    let get = match GetResponse::from_json(&get) {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    println!("Cobalt version {} @commit {}", get.cobalt.version, get.git.commit);
    println!("");
}

async fn tunnel_response(response: TunnelResponse) {
    let response = match reqwest::get(&response.url).await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    let response = match response.bytes().await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    match std::fs::write("test.mp3", response) {
        Ok(ok) => ok,
        Err(err) => todo!(),
    }
}

async fn post_entrance_audio(client: &Client) {
    const BODY: &str = stringify!(
        {
            "url": "https://www.youtube.com/watch?v=drxcdo8tH2Q",
            "downloadMode": "audio",
            "filenameStyle": "nerdy"
        }
    );

    println!("Attempting download of test content, making a POST request");
    let response = match post_cobalt(client, URL, BODY).await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    let response = match response.text().await {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    let response = match deserialize_post(&response) {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    println!("Recieved response");
    match response {
        PostResponse::Error(response) => todo!(),
        PostResponse::Picker(response) => todo!(),
        PostResponse::Redirect(response) => tunnel_response(response).await,
        PostResponse::Tunnel(response) => tunnel_response(response).await,
    };
}