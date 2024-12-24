use super::ticket::Ticket;

use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct TicketMacro {
    cobalt_filename: Option<bool>,
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

fn override_none<T>(target_state: &Option<T>, override_val: Option<T>) -> Option<T>{
    match target_state {
        Some(_) => return None, // Value exists, will not override
        None => match override_val { // Value does not exist, will attempt to override using overrid_val
            Some(val) => return Some(val),
            None => return None,
        }, 
    }
}

pub(crate) fn apply_ticket_macro(ticket_macro: &TicketMacro, ticket: &mut Ticket) {
    let ticket_state: &Ticket = &ticket.clone();

    ticket.cobalt_filename = override_none(&ticket_state.cobalt_filename, ticket_macro.cobalt_filename.clone());
    ticket.video_quality = override_none(&ticket_state.video_quality, ticket_macro.video_quality.clone());
    ticket.audio_format = override_none(&ticket_state.audio_format, ticket_macro.audio_format.clone());
    ticket.audio_bitrate = override_none(&ticket_state.audio_bitrate, ticket_macro.audio_bitrate.clone());
    ticket.filename_style = override_none(&ticket_state.filename_style, ticket_macro.filename_style.clone());
    ticket.download_mode = override_none(&ticket_state.download_mode, ticket_macro.download_mode.clone());
    ticket.youtube_video_codec = override_none(&ticket_state.youtube_video_codec, ticket_macro.youtube_video_codec.clone());
    ticket.youtube_dub_lang = override_none(&ticket_state.youtube_dub_lang, ticket_macro.youtube_dub_lang.clone());
    ticket.youtube_dub_browser_lang = override_none(&ticket_state.youtube_dub_browser_lang, ticket_macro.youtube_dub_browser_lang.clone());
    ticket.always_proxy = override_none(&ticket_state.always_proxy, ticket_macro.always_proxy.clone());
    ticket.disable_metadata = override_none(&ticket_state.disable_metadata, ticket_macro.disable_metadata.clone());
    ticket.tiktok_full_audio = override_none(&ticket_state.tiktok_full_audio, ticket_macro.tiktok_full_audio.clone());
    ticket.tiktok_h265 = override_none(&ticket_state.tiktok_h265, ticket_macro.tiktok_h265.clone());
    ticket.twitter_gif = override_none(&ticket_state.twitter_gif, ticket_macro.twitter_gif.clone());
}