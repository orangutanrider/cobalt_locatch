use locatch_macro::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub cobalt_url: String,
}
impl_from_json!(Config);