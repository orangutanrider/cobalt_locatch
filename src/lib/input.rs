//! File input

use serde::{ Deserialize, Serialize };
use serde_json::Error as JsonError;
use log::warn;

#[derive(Deserialize)]
pub struct SerialInput {
    settings: Option<SerialRequestMacro>,
    requests: Vec<SerialRequest>,
}
impl SerialInput {
    #[inline]
    fn from_json(json:&str) -> Result<Self, JsonError> {
        return serde_json::de::from_str::<Self>(json);
    }
}

type SerialRequestMacro = SerialRequest;
#[derive(Deserialize, Serialize)]
pub struct SerialRequest {
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
    pub fn apply_macro(mut self, settings: &SerialRequest) {
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

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let input = stringify!(
            {
                "requests":[]
            }
        );

        match SerialInput::from_json(input) {
            Ok(_) => return /* Test succesful */,
            Err(_) => panic!("Failed to deserialize"),
        };
    }

    #[test]
    fn empty_json() {
        let input = stringify!(
            {

            }
        );

        match SerialInput::from_json(input) {
            Ok(_) => panic!("Expected to be unable to deserialze, but did deserialize"),
            Err(_) => return /* Test succesful */,
        };
    }

    #[test]
    fn simple_input() {
        let input = stringify!(
            {
                "requests":[
                    {
                        "url":"https://www.youtube.com/watch?v=YgBaf3onLWs"
                    }
                ]
            }
        );

        match SerialInput::from_json(input) {
            Ok(_) => return /* Test succesful */,
            Err(_) => panic!("Failed to deserialize"),
        };
    }
}
