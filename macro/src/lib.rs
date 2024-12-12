use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

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