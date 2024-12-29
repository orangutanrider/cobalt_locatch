use reqwest::Client;

use tokio::runtime::Runtime;
use std::future::*;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;
use std::pin::*;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;

fn into_downloads(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    todo!()
}

fn download_unlimited(
    client: &Client, cobalt_url: &str,
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    todo!()
}

type DownloadSpool = Arc<Mutex<TicketMaster>>;

struct TicketMaster {
    tickets: std::vec::IntoIter<Ticket>,
    results: Vec<Result<(), LocatchErr>>,
}

fn download_with_limit(
    runtime: &mut Runtime,
    client: &Client, cobalt_url: &str, limit: usize, 
    tickets: Vec<Ticket>,
) -> Vec<Result<(), LocatchErr>> { 
    let output: Vec<Result<(), LocatchErr>> = Vec::with_capacity(tickets.len());
    let spool = Arc::new(Mutex::new(
        (tickets.into_iter(), output)
    ));

    todo!()
}

async fn download_thread(
    spool: DownloadSpool,
    client: &Client, cobalt_url: &str,
    pending: PendingDownload!(),
) {
    let result = pending.await;

    let mut lock = match spool.lock() {
        Ok(ok) => ok,
        Err(err) => {
            // todo log
            spool.clear_poison();
            spool.lock().unwrap()
        },
    };

    lock.results.push(result);
    let Some(ticket) = lock.tickets.next() else {
        return;
    };
    drop(lock);

    return download_thread(spool, client, cobalt_url, into_download(ticket, client, cobalt_url)).await
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
