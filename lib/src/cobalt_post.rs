//! Cobalt response representation and post

use locatch_macro::*;

use std::{
    str::FromStr,
    future::Future,
};

use serde::Deserialize;
use serde_json::Number;
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
    Response,
};

#[inline]
pub fn post_cobalt<T: Into<reqwest::Body>>(client: &Client, cobalt_url: &str, body: T) -> PendingResponse!() { 
    return client.post(cobalt_url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();
}

pub fn filter_responses(
    iter: std::vec::IntoIter<PostResponse>,
    errors: &mut Vec<ErrorResponse>,
    pickers: &mut Vec<PickerResponse>,
    tunnels: &mut Vec<TunnelResponse>,
) {
    for response in iter {
        match response {
            PostResponse::Error(error_response) => errors.push(error_response),
            PostResponse::Picker(picker_response) => pickers.push(picker_response),
            PostResponse::Redirect(tunnel_response) => tunnels.push(tunnel_response),
            PostResponse::Tunnel(tunnel_response) => tunnels.push(tunnel_response),
        }
    }
}

#[derive(Deserialize)]
pub enum PostResponse {
    Error(ErrorResponse),
    Picker(PickerResponse),
    Redirect(RedirectResponse),
    Tunnel(TunnelResponse),
}
impl_from_str_json!(PostResponse);

enum Status {
    Error,
    Picker,
    Redirect,
    Tunnel
}
impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => return Ok(Self::Error),
            "picker" => return Ok(Self::Picker),
            "redirect" => return Ok(Self::Redirect),
            "tunnel" => return Ok(Self::Tunnel),
            _ => return Err(())
        }
    }
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    #[allow(dead_code)]
    status: String,
    pub error: ErrorObj,
}

#[derive(Deserialize)]
pub struct ErrorObj {
    pub code: String,
    pub context: Option<ErrorContextObj>,
}

#[derive(Deserialize)]
pub struct ErrorContextObj {
    pub service: Option<String>,
    pub limit: Option<Number>
}

type RedirectResponse = TunnelResponse;
#[derive(Deserialize)]
pub struct TunnelResponse {
    #[allow(dead_code)]
    status: String,
    pub url: String,
    pub filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickerResponse {
    #[allow(dead_code)]
    status: String,
    pub audio: Option<String>,
    pub audio_filename: Option<String>,
    pub picker: Vec<PickerObj>,
}

pub enum PickerType {
    Photo,
    Video,
    Gif,
} 
impl FromStr for PickerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "photo" => return Ok(Self::Photo),
            "video" => return Ok(Self::Video),
            "gif" => return Ok(Self::Gif),
            _ => return Err(())
        }
    }
}

#[derive(Deserialize)]
pub struct PickerObj {
    /// key "type", maps to PickerType.
    pub kind: String,
    pub url: String, 
    pub thumb: Option<String>,
}