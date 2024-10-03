//! Cobalt response representation

use std::{
    str::FromStr,
    collections::HashMap,
    future::Future,
};

use serde::Deserialize;
use serde_json::{Number, Value};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
    Response,
    Error as ReqError
};

macro_rules! into_response {($enum:ident, $t:ty, $json:ident) => {{
        let response: $t = match serde_json::from_str($json) {
            Ok(ok) => ok,
            Err(_) => return Err(()),
        };
        return Ok(PostResponse::$enum(response));
}};}

#[inline]
pub fn post_cobalt<T: Into<reqwest::Body>>(client: &Client, cobalt_url: &str, body: T) -> impl Future<Output = Result<Response, ReqError>> { 
    return client.post(cobalt_url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();
}

pub fn deserialize_post(json_response: &str) -> Result<PostResponse, ()> {
    let lookup: HashMap<String, Value> = match serde_json::from_str(json_response) {
        Ok(ok) => ok,
        Err(_) => return Err(()),
    };

    let status = match lookup.get("status") {
        Some(status) => status,
        None => return Err(()),
    };
    let status = match status.as_str() {
        Some(status) => status,
        None => return Err(()),
    };
    let status = Status::from_str(status)?;

    match status {
        Status::Error => into_response!(Error, ErrorResponse, json_response),
        Status::Picker => into_response!(Picker, PickerResponse, json_response),
        Status::Redirect => into_response!(Redirect, RedirectResponse, json_response),
        Status::Tunnel => into_response!(Tunnel, TunnelResponse, json_response),
    }
} 

pub enum PostResponse {
    Error(ErrorResponse),
    Picker(PickerResponse),
    Redirect(RedirectResponse),
    Tunnel(TunnelResponse),
}

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
    status: String,
    pub url: String,
    pub filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickerResponse {
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