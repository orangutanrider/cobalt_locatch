use serde::{ Deserialize, Serialize };

pub(crate) mod default {
    pub(crate) fn video_quality() -> String            { "1080".to_owned() }
    pub(crate) fn audio_format() -> String             { "mp3".to_owned() }
    pub(crate) fn audio_bitrate() -> String            { "128".to_owned() }
    pub(crate) fn filename_style() -> String           { "classic".to_owned() }
    pub(crate) fn download_mode() -> String            { "auto".to_owned() }
    pub(crate) fn youtube_video_codec() -> String      { "h264".to_owned()}
    pub(crate) fn youtube_dub_lang() -> String         { "".to_owned() }
    pub(crate) fn youtube_dub_browser_lang() -> bool   { false }
    pub(crate) fn always_proxy() -> bool               { false }
    pub(crate) fn disable_metadata() -> bool           { false }
    pub(crate) fn tiktok_full_audio() -> bool          { false }
    pub(crate) fn tiktok_h265() -> bool                { false }
    pub(crate) fn twitter_gif() -> bool                { true }
}

#[derive(Deserialize)]
pub(crate) struct SerialInput {
    settings: Option<SerialSettings>,
    requests: Vec<SerialRequestBody>,
}

#[derive(Deserialize)]
/// Macro settings for an input
pub(crate) struct SerialSettings {
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

#[derive(Deserialize)]
/// Full body input.
/// Url with settings, will override macro settings.
pub(crate) struct SerialRequestBody {
    url: String,
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


// This crate: <https://crates.io/crates/derivative> could maybe be used to make this cleaner.
#[derive(Serialize)]
pub(crate) struct RequestBody {
    url: String,
    video_quality: String,
    audio_format: String,
    audio_bitrate: String,
    filename_style: String,
    download_mode: String,
    youtube_video_codec: String,
    youtube_dub_lang: String,
    youtube_dub_browser_lang: bool,
    always_proxy: bool,
    disable_metadata: bool,
    tiktok_full_audio: bool,
    tiktok_h265: bool,
    twitter_gif: bool,
}

macro_rules! from_value {($value:ident, $param:ident) => {
    match $value.$param {
        Some(v) => v,
        None => default::$param(),
    }
};}

macro_rules! overridable {($settings:ident, $body:ident, $param:ident) => {
    match $body.$param {
        Some(v) => v,
        None => match $settings.$param {
            Some(v) => v,
            None => default::$param(),
        },
    }
};}

impl RequestBody {
    pub(crate) fn to_json(&self) -> serde_json::Result<String> {
        serde_json::ser::to_string(self)
    }

    pub(crate) fn from_input(settings: Option<SerialSettings>, body: SerialRequestBody) -> Self {
        let Some(settings) = settings else {
            return Self::from(body);
        };

        return Self {
            url: body.url,
            video_quality: overridable!(settings, body, video_quality),
            audio_format: overridable!(settings, body, audio_format),
            audio_bitrate: overridable!(settings, body, audio_bitrate),
            filename_style: overridable!(settings, body, filename_style),
            download_mode: overridable!(settings, body, download_mode),
            youtube_video_codec: overridable!(settings, body, youtube_video_codec),
            youtube_dub_lang: overridable!(settings, body, youtube_dub_lang),
            youtube_dub_browser_lang: overridable!(settings, body, youtube_dub_browser_lang),
            always_proxy: overridable!(settings, body, always_proxy),
            disable_metadata: overridable!(settings, body, disable_metadata),
            tiktok_full_audio: overridable!(settings, body, tiktok_full_audio),
            tiktok_h265: overridable!(settings, body, tiktok_h265),
            twitter_gif: overridable!(settings, body, twitter_gif),
        }
    }
}
impl From<SerialRequestBody> for RequestBody {
    fn from(value: SerialRequestBody) -> Self {
        return Self {
            url: value.url,
            video_quality: from_value!(value, video_quality),
            audio_format: from_value!(value, audio_format),
            audio_bitrate: from_value!(value, audio_bitrate),
            filename_style: from_value!(value, filename_style),
            download_mode: from_value!(value, download_mode),
            youtube_video_codec:from_value!(value, youtube_video_codec),
            youtube_dub_lang: from_value!(value, youtube_dub_lang),
            youtube_dub_browser_lang: from_value!(value, youtube_dub_browser_lang),
            always_proxy: from_value!(value, always_proxy),
            disable_metadata: from_value!(value, disable_metadata),
            tiktok_full_audio: from_value!(value, tiktok_full_audio),
            tiktok_h265: from_value!(value, tiktok_h265),
            twitter_gif: from_value!(value, twitter_gif),
        }
    }
}
impl Default for RequestBody {
    fn default() -> Self {
        return Self { 
            url: "".to_owned(), 
            video_quality: default::video_quality(), 
            audio_format: default::audio_format(),
            audio_bitrate: default::audio_bitrate(), 
            filename_style: default::filename_style(), 
            download_mode: default::download_mode(), 
            youtube_video_codec: default::youtube_video_codec(), 
            youtube_dub_lang: default::youtube_dub_lang(),
            youtube_dub_browser_lang: default::youtube_dub_browser_lang(),
            always_proxy: default::always_proxy(),
            disable_metadata: default::disable_metadata(),
            tiktok_full_audio: default::tiktok_full_audio(),
            tiktok_h265: default::tiktok_h265(),
            twitter_gif: default::twitter_gif(),
        }
    }
}