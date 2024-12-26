use locatch_lib::*;
use locatch_macro::PendingDownload;

use core::slice;
use std::future::Future;
use reqwest::Client;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;
use crate::sanitize::*;

async fn into_download(ticket: Ticket, client: &Client, cobalt_url: &str) -> Result<(), LocatchErr> {
    // post
    let (response, ticket) = request(client, cobalt_url, ticket).await;

    let response = match response {
        Ok(ok) => ok,
        Err(err) => todo!(),
    };

    match response {
        PostResponse::Redirect(tunnel) => return tunnel_download(client, tunnel, ticket).await,
        PostResponse::Tunnel(tunnel) => return tunnel_download(client, tunnel, ticket).await,
        PostResponse::Picker(picker) => todo!(),
        PostResponse::Error(error) => todo!(),
    }
}

async fn tunnel_download(client: &Client, tunnel: TunnelResponse, ticket: SentTicket) -> Result<(), LocatchErr> {
    let filename = match ticket.filename {
        Some(filename) => filename,
        None => tunnel.filename,
    };
    
    let filename = sanitize_filename::sanitize(filename);
    return download(client, &tunnel.url, &filename).await
}

/* 
#[inline]
pub async fn post_office(client: &Client, cobalt_url: &str, list: List) {
    match config.concurrent_download_limit {
        Some(limit) => {
            return limited_post_office(client, cobalt_url, list, limit).await
        },
        None => {
            return no_limit_post_office(client, cobalt_url, list).await
        },
    }
}

#[inline]
async fn limited_post_office(client: &Client, cobalt_url: &str, list: List, limit: usize) {

}

#[inline]
async fn no_limit_post_office(client: &Client, cobalt_url: &str, list: List) {
    for ticket in list.tickets.into_iter() {
        // post
        let (response, ticket) = post_request(client, &config.cobalt_url, ticket).await;

    }
}
*/

/* 
#[inline]
pub async fn post_office(client: &Client, config: Config, list: List) {
    let len = list.tickets.len();

    // post
    let (responses, tickets) = make_requests(&client, &config.cobalt_url, list, len);
    let responses = unwrap_responses(responses, len).await;

    // request texts
    let responses = request_response_texts(responses, len);
    let responses = unwrap_pending_texts(responses, len).await;

    // deserialzie
    let responses = deserialize_responses(responses, len);

    // filter
    let mut errors = Vec::with_capacity(len);
    let mut pickers = Vec::with_capacity(len);
    let mut tunnels = Vec::with_capacity(len);
    filter_responses(responses.into_iter(), &mut errors, &mut pickers, &mut tunnels);

    // sanitize
    let tunnels_sanitize = tunnels_sanitize(&mut tunnels);
    // pickers are todo
    tunnels_sanitize.await;
    // pickers_sanitize.await; 
}
*/

/* 
/// Outputs a vec of PendingDownload
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

/// Returns the number of failed downloads
pub async fn await_downloads(downloads: Vec<PendingDownload!()>) -> usize {
    let mut fail_count: usize = 0;

    for download in downloads.into_iter() {
        match download.await {
            Ok(_) => {/* Do nothing */},
            Err(err) => {
                fail_count = fail_count + 1;
                println!("Encountered an error with a download: \n{}", err)
            },
        }
    }

    return fail_count;
}
*/

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