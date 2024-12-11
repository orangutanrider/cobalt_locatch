// relocate to process

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SerialConfig {
    pub cobalt_url: String,
}
impl_from_json!(SerialConfig);