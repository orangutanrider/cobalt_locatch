use lib::*;
use reqwest::Client;

const URL: &str = "http://localhost:9000";

/// If the bytes recieved are less than this, the system will think that something has gone wrong, and will provide additional information about what happened.
const LEN_ERR_THRESHOLD: usize = 1000;

const BODY: &str = stringify!(
    {
        "url": "https://www.youtube.com/watch?v=drxcdo8tH2Q", // Creaks entrance music
        "downloadMode": "audio",
        "filenameStyle": "nerdy"
    }
);

fn main() {
    println!("Starting download test");
    println!("Url is {}", URL);

    let async_runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build() 
    {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to start tokio async runtime");
            panic!("{}", err);
        },
    };

    async_runtime.block_on(get_cobalt());

    let client = Client::new();

    async_runtime.block_on(post_entrance_audio(&client));
}

async fn get_cobalt() {
    println!("Attempting to connect, making a GET request");
    let get = match reqwest::get(URL).await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to connect");
            panic!("Error: {}", err);
        },
    };
    println!("Succesfully connected");

    let get = match get.text().await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to get response text");
            panic!("Error: {}", err);
        },
    };

    let get = match GetResponse::from_json(&get) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to convert response text json to internal data structure");
            panic!("Error: {}", err);
        },
    };

    println!("Cobalt version {} @commit {}", get.cobalt.version, get.git.commit);
    println!("");
}

async fn tunnel_response(response: TunnelResponse) {
    println!("{}", response.filename);
    println!("{}", response.url);
    let response = match reqwest::get(&response.url).await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to connect to response url");
            panic!("Error: {}", err);
        },
    };

    let response = match response.bytes().await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to get response bytes");
            panic!("Error: {}", err);
        },
    };

    println!("response len is {}", response.len());

    if response.len() < LEN_ERR_THRESHOLD {
        println!("Unexpectedly low size for response bytes, possibly recieved an error");
        println!("This could indicate that cobalt has been configured incorrectly");
        println!("For this test, make sure that the API_URL is the same as \"{}\"", URL);
        println!("Writing an additional text file, using the bytes, inspect it to see if what was recieved was an error message");

        match std::fs::write("api_test.txt", &response) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Couldn't write the bytes to api_test.txt");
                println!("Error: {}", err);
            },
        }
    }

    match std::fs::write("api_test.mp3", &response) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Couldn't write the bytes to api_test.mp3");
            panic!("Error: {}", err);
        },
    }
}

async fn post_entrance_audio(client: &Client) {
    println!("Attempting download of test content, making a POST request");
    let response = match post_cobalt(client, URL, BODY).await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to post request to cobalt");
            panic!("Error: {}", err);
        },
    };

    let response = match response.text().await {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to get response text");
            panic!("Error: {}", err);
        },
    };

    let response = match deserialize_post(&response) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Failed to deserialze response");
            panic!("Error: {:?}", err);
        },
    };

    println!("Recieved response");
    match response {
        PostResponse::Error(response) => {
            println!("Error");
            println!("Code: {}", response.error.code);
            match response.error.context {
                Some(error_context) => {
                    match error_context.limit {
                        Some(limit) => println!("Limit: {}", limit),
                        None => {/* Do nothing */},
                    }
                    match error_context.service {
                        Some(service) => println!("Service: {}", service),
                        None => {/* Do nothing */},
                    }
                },
                None => {/* Do nothing */},
            }
            panic!("Error is unsexpected for the download test");
        },
        PostResponse::Picker(_) => {
            println!("Picker");
            panic!("Picker is unexpected for the download test");
        },
        PostResponse::Redirect(response) => {
            println!("Redirect");
            tunnel_response(response).await
        },
        PostResponse::Tunnel(response) => {
            println!("Tunnel");
            tunnel_response(response).await
        },
    };
}