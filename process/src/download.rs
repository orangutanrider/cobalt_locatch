use reqwest::Client;
use std::future::*;
use std::task::Context;
use std::task::Poll;
use std::pin::*;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;

async fn into_downloads(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    match concurrent_download_limit {
        Some(limit) => download_with_limit(client, cobalt_url, limit, tickets).await,
        None => download_unlimited(client, cobalt_url, tickets).await,
    }
}

async fn download_unlimited(
    client: &Client, cobalt_url: &str,
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    let mut pending_downloads = Vec::with_capacity(tickets.len());
    let mut output = Vec::with_capacity(tickets.len());

    for ticket in tickets.into_iter() {
        pending_downloads.push(into_download(ticket, client, cobalt_url));
    }

    for pending in pending_downloads.into_iter() {
        output.push(pending.await)
    }

    return output
}
 
#[inline]
fn iter_pin_mut<T>(slice: Pin<&mut [T]>) -> impl Iterator<Item = Pin<&mut T>> {
    return unsafe { slice.get_unchecked_mut() }
        .iter_mut()
        .map(|t| unsafe { Pin::new_unchecked(t) })
}

async fn download_with_limit(
    client: &Client, cobalt_url: &str, limit: usize, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> { 
    let len = tickets.len();
    let mut output_vec: Vec<Result<(), LocatchErr>> = Vec::with_capacity(len);
    let mut open_vec: Vec<usize> = Vec::with_capacity(limit);
    let mut poll_vec = Vec::with_capacity(limit);
    let mut tickets = tickets.into_iter();

    // init
    let mut init: usize = 0;
    while init != limit {
        let Some(ticket) = tickets.next() else {
            break;
        }; 
        
        poll_vec.push(into_download(ticket, client, cobalt_url));

        init = init + 1;
    }

    let mut poll_vec = poll_vec.into_boxed_slice();
    let mut poll_vec = unsafe { Pin::new_unchecked(poll_vec.as_mut()) };

    // process


    todo!()
}

#[inline]
// poll_fn
fn download_with_limit_process(
    cx: &mut Context,
    output_vec: Vec<Result<(), LocatchErr>>, open_vec: Vec<usize>, poll_vec: Pin<&mut [PendingDownload!()]>,
    tickets: std::vec::IntoIter<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    /* 
    loop
    poll the poll vec
    if download finished
        push the index of its position in the poll vec to the open vec
        push its result to the output vec
    for each index in open vec
        start a download by popping the tickets vec
        replace the entry at the index in the poll vec with the new download
    clear the open vec
    */
    todo!()
}

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
        Some(filename) => match ticket.cobalt_filename {
            true => tunnel.filename,
            false => filename,
        },
        None => tunnel.filename,
    };
    
    let filename = sanitize_filename::sanitize(filename);
    return download(client, &tunnel.url, &filename).await
}
