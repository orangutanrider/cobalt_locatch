use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::{Ticket, SentTicket};

use reqwest::Client;

pub(crate) async fn request(
    client: &Client, cobalt_url: &str, ticket: Ticket
) -> (Result<PostResponse, LocatchErr>, SentTicket) {
    let (sent, request) = ticket.to_send();

    let response = match request.to_json() {
        Ok(body) => post_cobalt(client, cobalt_url, body).await, // send
        Err(err) => {
            return (Err(LocatchErr::Json(err)), sent)
        },
    };

    // unwrap
    let response = match response {
        Ok(ok) => ok,
        Err(err) => return (Err(LocatchErr::Req(err)), sent),
    };

    // text
    let response = response.text().await;

    // unwrap
    let response = match response {
        Ok(ok) => ok,
        Err(err) => return (Err(LocatchErr::Req(err)), sent),
    };

    // deserialize
    let response = match PostResponse::from_json(&response) {
        Ok(ok) => ok,
        Err(err) => return (Err(LocatchErr::Json(err)), sent),
    };

    return (Ok(response), sent)
}

/* 
pub async fn get_cobalt(client: &Client, cobalt_url: &str) -> Result<(), ReqError> {
    let response = match client.get(cobalt_url).send().await {
        Ok(ok) => {
            println!("Succesfully connected to cobalt"); // verbose log
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
            println!("Error: {}", err); // verbose log
            println!("Couldn't get the response text from cobalt"); // verbose log
            println!("Since cobalt was succesfully connected to, will try to continue execution anyways"); // verbose log
            return Ok(())
        },
    };

    let response = match GetResponse::from_json(&response) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error: {}", err); // verbose log
            println!("Failed to deserialize cobalt response"); // verbose log
            println!("This could indicate that an incompatible version of cobalt is being connected to"); // verbose log
            println!("Will try to continue execution anyways"); // verbose log
            return Ok(())
        },
    };

    println!("Cobalt version {} @commit {}", response.cobalt.version, response.git.commit);
    Ok(())
}

pub fn make_requests(
    client: &Client, cobalt_url: &str, list: List, len: usize,
) -> (Vec<PendingResponse!()>, Vec<SentTicket>) {
    let mut futures = Vec::with_capacity(len);
    let mut ticket_output = Vec::with_capacity(len);
    
    // simd
    for ticket in list.tickets.into_iter() { 
        let (sent, request) = ticket.to_send();
        ticket_output.push(sent);

        match request.to_json() {
            Ok(body) => futures.push(post_cobalt(client, cobalt_url, body)),
            Err(err) => {
                println!("A request could not be serialized, error: {}", err);
                continue;
            },
        };
    }
    
    return (futures, ticket_output);
}

pub async fn unwrap_responses(requests: Vec<PendingResponse!()>, len: usize) -> Vec<Response> {
    let mut responses = Vec::with_capacity(len);

    // simd
    for future in requests.into_iter() {
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

pub fn deserialize_responses(responses: Vec<String>, len: usize) -> Vec<PostResponse> {
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

pub fn request_response_texts(responses: Vec<Response>, len: usize) -> Vec<PendingText!()> {
    let mut futures = Vec::with_capacity(len);

    for response in responses.into_iter() { // par SIMD possible?
        futures.push(response.text());
    }

    return futures;
}

pub async fn unwrap_pending_texts(pending_texts: Vec<PendingText!()>, len: usize) -> Vec<String> {
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
*/