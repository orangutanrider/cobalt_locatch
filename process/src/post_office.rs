//! Sends requests to cobalt, and processes responses from cobalt.

use reqwest::Client;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;
use crate::sanitize::*;

#[inline]
pub async fn post_office(client: &Client, config: Config, list: List) {
    let len = list.tickets.len();

    // post
    let (responses, tickets) = make_requests(&client, &config.cobalt_url, list, len);
    let responses = unwrap_responses(responses, len).await;

    // request texts
    let responses = request_response_texts(responses, len);
    let responses = unwrap_pending_texts(responses, len).await;

    // deserialzie
    let responses = deserialize_responses(responses, len);

    // filter
    let mut errors = Vec::with_capacity(len);
    let mut pickers = Vec::with_capacity(len);
    let mut tunnels = Vec::with_capacity(len);
    filter_responses(responses.into_iter(), &mut errors, &mut pickers, &mut tunnels);

    // sanitize
    let tunnels_sanitize = tunnels_sanitize(&mut tunnels);
    // pickers are todo
    tunnels_sanitize.await;
    // pickers_sanitize.await;

}