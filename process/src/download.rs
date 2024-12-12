use locatch_lib::*;

use core::slice;
use std::future::Future;
use reqwest::Client;

pub fn start_download_tunnels<'a>(
    client: &'a Client,
    iter: slice::Iter<'a, TunnelResponse>, 
    len: usize
) -> Vec<impl Future<Output = Result<(), DownloadError>> + use<'a>> {
    let mut futures = Vec::with_capacity(len);
    
    for tunnel in iter { 
        futures.push(download(client, &tunnel.url, &tunnel.filename));
    }

    return futures;
}

// https://github.com/lostdusty/retrobalt/blob/main/module_downloader.go#L22-L49
// https://discord.com/channels/1049706293700591677/1049740077460377660/1291849923519578154
//async fn start_download_pickers<'a>(
//    iter: slice::Iter<'a, PickerResponse>, 
//    len: usize
//) -> Vec<impl Future<Output = Result<(), DownloadError>> + use<'a>> {
//    todo!("Handling picker respones is un-implemented");
//
//    //todo!("Parsing filenames from headers to-be-implemented");
//
//    let mut futures = Vec::with_capacity(len);
//
//    let mut outer_index = 0;
//    for picker in iter { 
//        let mut inner_index = 0;
//        for picker_obj in picker.picker.iter() {
//            
//            inner_index = inner_index + 1;
//        }
//
//        let Some(aud_url) = picker.audio else {
//            outer_index = outer_index + 1;
//            continue;
//        };
//
//        let Some(aud_filename) = picker.audio_filename else {
//            println!("Recieved an audio url, without recieving its filename"); 
//            println!("Logging unimplemented"); //todo!
//            //warn!("");
//
//            outer_index = outer_index + 1;
//            continue;
//        };
//
//        outer_index = outer_index + 1;
//    }
//
//    return futures;
//}