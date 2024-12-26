use locatch_macro::*;
use locatch_lib::CobaltRequest;

use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone, Default)]
pub struct Ticket {
    pub url: String,
    pub filename: Option<String>,
    pub cobalt_filename: Option<bool>,
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
impl_from_json!(Ticket);
impl Ticket {
    pub fn to_send(self) -> (SentTicket, CobaltRequest) {
        return (
            SentTicket{
                filename: self.filename.clone(),
                cobalt_filename: match self.cobalt_filename {
                    Some(val) => val,
                    None => false,
                },
            },
            CobaltRequest {
                url: Some(self.url),
                video_quality: self.video_quality,
                audio_format: self.audio_format,
                audio_bitrate: self.audio_bitrate,
                filename_style: self.filename_style,
                download_mode: self.download_mode,
                youtube_video_codec: self.youtube_video_codec,
                youtube_dub_lang: self.youtube_dub_lang,
                youtube_dub_browser_lang: self.youtube_dub_browser_lang,
                always_proxy: self.always_proxy,
                disable_metadata: self.disable_metadata,
                tiktok_full_audio: self.tiktok_full_audio,
                tiktok_h265: self.tiktok_h265,
                twitter_gif: self.twitter_gif,
            }
        )
    }
}

pub struct SentTicket {
    pub filename: Option<String>,
    pub cobalt_filename: bool,
}
