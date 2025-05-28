use locatch_macro::*;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub struct CobaltRequest {
    pub url: Option<String>,
    pub video_quality: Option<String>,
    pub audio_format: Option<String>,
    pub audio_bitrate: Option<String>,
    pub filename_style: Option<String>,
    pub download_mode: Option<String>,
    pub youtube_video_codec: Option<String>,
    pub youtube_dub_lang: Option<String>,
    pub youtube_dub_browser_lang: Option<bool>,
    pub always_proxy: Option<bool>,
    pub disable_metadata: Option<bool>,
    pub tiktok_full_audio: Option<bool>,
    pub tiktok_h265: Option<bool>,
    pub twitter_gif: Option<bool>,
}
impl_to_json!(CobaltRequest);