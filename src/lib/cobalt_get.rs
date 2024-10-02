use serde::Deserialize;
use serde_json::Number;
use serde_json::Error as JsonError;

#[derive(Deserialize)]
pub struct GetResponse {
    pub cobalt: CobaltObj,
    pub git: GitObj,
}
impl GetResponse {
    #[inline]
    pub fn from_json(json:&str) -> Result<Self, JsonError> {
        return serde_json::de::from_str::<Self>(json);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobaltObj {
    pub version: String,
    pub url: String,
    pub start_time: String,
    pub duration_limit: Number,
    pub services: Vec<String>,
}

#[derive(Deserialize)]
pub struct GitObj {
    pub commit: String,
    pub branch: String,
    pub remote: String,
}