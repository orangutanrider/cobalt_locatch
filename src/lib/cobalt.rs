use std::str::FromStr;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Number, Value};

macro_rules! into_cobalt_response {($enum:ident, $t:ty, $json:ident) => {{
        let response: $t = match serde_json::from_str($json) {
            Ok(ok) => ok,
            Err(_) => return Err(()),
        };
        return Ok(CobaltResponse::$enum(response));
}};}

pub(crate) fn deserialize_cobalt(json_response: &str) -> Result<CobaltResponse, ()> {
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
        Status::Error => into_cobalt_response!(Error, ErrorResponse, json_response),
        Status::Picker => into_cobalt_response!(Picker, PickerResponse, json_response),
        Status::Redirect => into_cobalt_response!(Redirect, RedirectResponse, json_response),
        Status::Tunnel => into_cobalt_response!(Tunnel, TunnelResponse, json_response),
    }
} 

pub(crate) enum CobaltResponse {
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
pub(crate) struct ErrorResponse {
    status: String,
    pub(crate) error: ErrorObj,
}

#[derive(Deserialize)]
pub(crate) struct ErrorObj {
    pub(crate) code: String,
    pub(crate) context: Option<ErrorContextObj>,
}

#[derive(Deserialize)]
pub(crate) struct ErrorContextObj {
    pub(crate) service: Option<String>,
    pub(crate) limit: Option<Number>
}

type RedirectResponse = TunnelResponse;
#[derive(Deserialize)]
pub(crate) struct TunnelResponse {
    status: String,
    pub(crate) url: String,
    pub(crate) filename: String,
}

#[derive(Deserialize)]
pub(crate) struct PickerResponse {
    status: String,
    pub(crate) audio: Option<String>,
    pub(crate) audio_filename: Option<String>,
    pub(crate) picker: Vec<PickerObj>,
}

pub(crate) enum PickerType {
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
pub(crate) struct PickerObj {
    /// key "type", maps to PickerType.
    pub(crate) kind: String,
    pub(crate) url: String, 
    pub(crate) thumb: Option<String>,
}