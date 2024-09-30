use crate::post_cobalt;

const URL: &str = "http://localhost:9000";

async fn get_cobalt_test() {
    let Ok(get) = reqwest::get(URL).await else {
        panic!("Failed to connect");
    };

    let Ok(get) = get.text().await else {
        panic!("Failed to get the response text");
    };

    // Succesfully made a GET request
}