use serde::{Deserialize, Serialize};
use std::future::Future;
use reqwest::{Client, Response};

pub mod unsafe_lib;

pub type IOError = std::io::Error;
pub type JsonError = serde_json::Error;
pub type ReqError = reqwest::Error;

pub enum LocatchErr {
    Io(IOError),
    Json(JsonError),
    Req(ReqError),
    Empty
}
impl std::fmt::Display for LocatchErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocatchErr::Io(error) => write!(f, "{}", error),
            LocatchErr::Json(error) => write!(f, "{}", error),
            LocatchErr::Req(error) => write!(f, "{}", error),
            LocatchErr::Empty => write!(f, "empty"),
        }
    }
}

pub trait ReqRequest: 'static { 
    fn request<T: Into<reqwest::Body>>(client: &Client, url: &str, body: T) -> impl Future<Output = Result<Response, ReqError>>;
}

pub trait FromJson<'de, T>: Deserialize<'de> {
    fn from_json(json: T) -> Result<Self, JsonError>;
}

/// &str json
#[macro_export]
macro_rules! impl_from_str_json {($type:ty) => {
    impl<'de> locatch_macro::FromJson<'de, &'de str> for $type {
        #[inline]
        fn from_json(json: &'de str) -> Result<Self, locatch_macro::JsonError> {
            return serde_json::de::from_str::<Self>(json);
        }
     }
};}

/// String json
#[macro_export]
macro_rules! impl_from_string_json {($type:ty) => {
    impl<'de> locatch_macro::FromJson<'de, String> for $type {
        fn from_json(json: String) -> Result<Self, locatch_macro::JsonError> {
            serde_json::de::from_str::<Self>(&json)
        }
    }
};}

pub trait ToJson: Serialize {
    #[inline]
    fn to_json(&self) -> Result<String, JsonError> {
        return serde_json::to_string(self)
    }
}
#[macro_export]
macro_rules! impl_to_json {($type:ty) => {
    impl locatch_macro::ToJson for $type { }
};}

#[macro_export]
macro_rules! PendingResponse {() => {
    impl Future<Output = Result<Response, ReqError>>
};}

#[macro_export]
macro_rules! PendingText {() => {
    impl Future<Output = Result<String, ReqError>>
};}

#[macro_export]
macro_rules! PendingDownload {() => {
    impl Future<Output = Result<(), LocatchErr>>
}}