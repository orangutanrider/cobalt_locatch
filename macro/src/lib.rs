use serde::{Deserialize, Serialize};
use reqwest::Error;

pub type IOError = std::io::Error;
pub type JsonError = serde_json::Error;
pub type ReqError = reqwest::Error;

pub trait FromJson<'de>: Deserialize<'de> {
    #[inline]
    fn from_json(json:&'de str) -> Result<Self, JsonError> {
        return serde_json::de::from_str::<Self>(json);
    }
}
#[macro_export]
macro_rules! impl_from_json {($type:ty) => {
    impl<'de> locatch_macro::FromJson<'_> for $type { }
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

// type PendingRequest = impl Future<Output = Result<Response, ReqError>>;
#[macro_export]
macro_rules! PendingResponse {() => {
    impl Future<Output = Result<Response, ReqError>>
};}


// type PendingText = impl Future<Output = Result<String, ReqError>>;
#[macro_export]
macro_rules! PendingText {() => {
    impl Future<Output = Result<String, ReqError>>
};}