use locatch_macro::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub cobalt_url: String,
    pub async_threads: Option<usize>,
    pub async_stack_size: Option<usize>,
    pub concurrent_download_limit: Option<usize>,
}
impl_from_json!(Config);