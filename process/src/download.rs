use reqwest::Client;
use std::future::*;
use std::pin::pin;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;

async fn into_downloads(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    match concurrent_download_limit {
        Some(limit) => download_with_limit(client, cobalt_url, limit, tickets),
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

fn download_with_limit(
    client: &Client, cobalt_url: &str, mut limit: usize, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    let mut open: usize = 0;
    let mut tickets = tickets.into_iter();

    let mut pending_downloads = Vec::new();
    let mut output = Vec::new();

    // init
    loop {
        let Some(ticket) = tickets.next() else {
            break;
        };

        // pending
        pending_downloads.push(into_download(ticket, client, cobalt_url));

        if limit == 0 {
            break;
        }
        else {
            limit = limit - 1;
        }
    }
    

    todo!()
}

async fn poll_downloads<'a>(    
    mut pending_downloads: std::vec::IntoIter<PendingDownload!()>,
    output: &mut Vec<Result<(), LocatchErr>>,
) {
    let Some(pending) = pending_downloads.next() else {
        return;
    };

    let pending = pin!(pending);
    

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
