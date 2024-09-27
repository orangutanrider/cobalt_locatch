use serde::{ Deserialize, Serialize };
use log::warn;

// pub(crate) mod default {
//     pub(crate) fn video_quality() -> String            { "1080".to_owned() }
//     pub(crate) fn audio_format() -> String             { "mp3".to_owned() }
//     pub(crate) fn audio_bitrate() -> String            { "128".to_owned() }
//     pub(crate) fn filename_style() -> String           { "classic".to_owned() }
//     pub(crate) fn download_mode() -> String            { "auto".to_owned() }
//     pub(crate) fn youtube_video_codec() -> String      { "h264".to_owned()}
//     pub(crate) fn youtube_dub_lang() -> String         { "".to_owned() }
//     pub(crate) fn youtube_dub_browser_lang() -> bool   { false }
//     pub(crate) fn always_proxy() -> bool               { false }
//     pub(crate) fn disable_metadata() -> bool           { false }
//     pub(crate) fn tiktok_full_audio() -> bool          { false }
//     pub(crate) fn tiktok_h265() -> bool                { false }
//     pub(crate) fn twitter_gif() -> bool                { true }
// }

#[derive(Deserialize)]
pub(crate) struct SerialInput {
    settings: Option<SerialRequestMacro>,
    requests: Vec<SerialRequest>,
}

type SerialRequestMacro = SerialRequest;
#[derive(Deserialize, Serialize)]
pub(crate) struct SerialRequest {
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

macro_rules! apply_field {($self:ident, $settings:ident, $field:ident) => {
    $self.$field = match $self.$field {
        Some(v) => Some(v),
        None => match &$settings.$field {
            Some(v) => Some(v.clone()),
            None => {
                None
            },
        },
    }
};}

impl SerialRequest {
    pub(crate) fn apply_macro(mut self, settings: &SerialRequest) {
        // apply url field
        self.url = match self.url {
            Some(v) => Some(v),
            None => match &settings.url {
                Some(v) => Some(v.clone()),
                None => {
                    warn!("There was an entry with no URL and no URL able to be given to it.");
                    None
                },
            },
        };

        apply_field!(self, settings, video_quality);
        apply_field!(self, settings, audio_format);
        apply_field!(self, settings, audio_bitrate);
        apply_field!(self, settings, filename_style);
        apply_field!(self, settings, download_mode);
        apply_field!(self, settings, youtube_video_codec);
        apply_field!(self, settings, youtube_dub_lang);
        apply_field!(self, settings, youtube_dub_browser_lang);
        apply_field!(self, settings, always_proxy);
        apply_field!(self, settings, disable_metadata);
        apply_field!(self, settings, tiktok_full_audio);
        apply_field!(self, settings, tiktok_h265);
        apply_field!(self, settings, twitter_gif);
    }

    pub(crate) fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}