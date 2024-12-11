// relocate to process

use serde::{ Deserialize, Serialize };
//use log::warn;

#[derive(Deserialize)]
pub struct SerialInput {
    #[serde(alias = "macro")]
    pub marco: Option<SerialRequestMacro>,
    pub requests: Vec<SerialRequest>,
}
impl_from_json!(SerialInput);
impl SerialInput {
    /// Apply the macro to the requests (if there is a macro)
    pub fn apply_macro(&mut self) {
        let Some(marco) = &self.marco else {
            // There is no macro to apply
            return;
        };

        for request in self.requests.iter_mut() {
            request.apply_macro(marco);
        }
    }

    // Thereotically more performant.
    // Instead of cloning state at each step of the iteration, the entire vec is simply cloned and then values are fed in as state.
    // Un-tested.
    pub fn apply_macro_vec_clone(&mut self) {
        let Some(marco) = &self.marco else {
            // There is no macro to apply
            return;
        };

        let mut state = self.requests.clone();

        let mut index = 0;
        while index < self.requests.len() {
            index = index + 1;
            let request_state = state.remove(index);
            self.requests[index].apply_macro_with(request_state, marco);
        }
    }

    /// Apply macro onto requests in parallel
    /// Un-implemented
    pub fn apply_macro_par() {
        todo!()
    }
}

type SerialRequestMacro = SerialRequest;
#[derive(Deserialize, Serialize)]
#[derive(Clone)]
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

macro_rules! apply_field {($self:ident, $state:ident, $marco:ident, $field:ident) => {
    $self.$field = match $state.$field {
        Some(v) => Some(v),
        None => match &$marco.$field {
            Some(v) => Some(v.clone()),
            None => {
                None
            },
        },
    }
};}

impl SerialRequest {
    pub fn apply_macro(&mut self, marco: &SerialRequest) {
        let state = self.clone();

        self.url = match state.url {
            Some(v) => Some(v),
            None => match &marco.url {
                Some(v) => Some(v.clone()),
                None => {
                    println!("There was an entry with no URL and no URL able to be given to it"); todo!("Logging unimplemented");
                    //warn!("There was an entry with no URL and no URL able to be given to it");
                    None
                },
            },
        };

        apply_field!(self, state, marco, video_quality);
        apply_field!(self, state, marco, audio_format);
        apply_field!(self, state, marco, audio_bitrate);
        apply_field!(self, state, marco, filename_style);
        apply_field!(self, state, marco, download_mode);
        apply_field!(self, state, marco, youtube_video_codec);
        apply_field!(self, state, marco, youtube_dub_lang);
        apply_field!(self, state, marco, youtube_dub_browser_lang);
        apply_field!(self, state, marco, always_proxy);
        apply_field!(self, state, marco, disable_metadata);
        apply_field!(self, state, marco, tiktok_full_audio);
        apply_field!(self, state, marco, tiktok_h265);
        apply_field!(self, state, marco, twitter_gif);
    }

    /// .apply_macro() clones itself, to hold its state before the macro was applied.
    /// It may be more efficient to clone elsewhere, rather than for each individual call of that method.
    pub fn apply_macro_with(&mut self, state: SerialRequest, marco: &SerialRequest) {
        self.url = match state.url {
            Some(v) => Some(v),
            None => match &marco.url {
                Some(v) => Some(v.clone()),
                None => {
                    println!("There was an entry with no URL and no URL able to be given to it"); todo!("Logging unimplemented");
                    //warn!("There was an entry with no URL and no URL able to be given to it");
                    None
                },
            },
        };

        apply_field!(self, state, marco, video_quality);
        apply_field!(self, state, marco, audio_format);
        apply_field!(self, state, marco, audio_bitrate);
        apply_field!(self, state, marco, filename_style);
        apply_field!(self, state, marco, download_mode);
        apply_field!(self, state, marco, youtube_video_codec);
        apply_field!(self, state, marco, youtube_dub_lang);
        apply_field!(self, state, marco, youtube_dub_browser_lang);
        apply_field!(self, state, marco, always_proxy);
        apply_field!(self, state, marco, disable_metadata);
        apply_field!(self, state, marco, tiktok_full_audio);
        apply_field!(self, state, marco, tiktok_h265);
        apply_field!(self, state, marco, twitter_gif);
    }
}
impl_to_json!(SerialRequest);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromJson;

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
