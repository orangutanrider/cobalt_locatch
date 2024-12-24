use locatch_macro::*;

use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub struct CobaltRequest {
    url: Option<String>,
    video_quality: Option<String>,
    audio_format: Option<String>,
    audio_bitrate: Option<String>,
    filename_style: Option<String>,
    download_mode: Option<String>,
    youtube_video_codec: Option<String>,
    youtube_dub_lang: Option<String>,
    youtube_dub_browser_lang: Option<bool>,
    always_proxy: Option<bool>,
    disable_metadata: Option<bool>,
    tiktok_full_audio: Option<bool>,
    tiktok_h265: Option<bool>,
    twitter_gif: Option<bool>,
}
impl_to_json!(CobaltRequest);