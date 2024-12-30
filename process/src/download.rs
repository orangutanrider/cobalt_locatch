use reqwest::Client;

use tokio::task::*;
use tokio::runtime::Runtime;
use std::future::*;
use std::sync::{Arc, Mutex};

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

type TicketSpool = (
    // tickets
    std::vec::IntoIter<Ticket>,
    // results
    Vec<Result<(), LocatchErr>>
);

async fn download_with_limit(
    runtime: &mut Runtime,
    client: &Client, cobalt_url: &str, limit: usize, 
    tickets: Vec<Ticket>,
) -> Vec<Result<(), LocatchErr>> { 
    let len = tickets.len();
    let mut tickets = tickets.into_iter();
    let mut init = Vec::with_capacity(limit);
    let output= Vec::with_capacity(len);

    for _ in 0..limit {
        let Some(ticket) = tickets.next() else {
            break;
        };

        init.push(ticket);
    }
    let init = init.into_iter();

    let mut spool = Mutex::new(
        (tickets.into_iter(), output)
    );

    let mut set = JoinSet::new();
    for ticket in init {
        set.spawn_local( unsafe { into_download_thread(&mut spool, client, cobalt_url, ticket) });
    }

    // await
    set.join_all().await;

    let (_, results) = match spool.into_inner() {
        Ok(ok) => ok,
        Err(_) => panic!(),
    };

    return results;
}

// An Arc isn't needed because the thread is joined before the scope ends.
/// It is expected that the thread is joined before the outer scope ends.
#[inline]
async unsafe fn into_download_thread(
    spool: *mut Mutex<TicketSpool>,
    client: *const Client, cobalt_url: *const str,
    ticket: Ticket
) {
    return download_thread(spool, client, cobalt_url, 
        into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked())
    ).await
}

// awaits the pending download.
// Adds the result to spool, and takes a ticket to start a new download.
// Continues until spool has no tickets.
/// It is expected that the thread is joined before the outer scope ends.
#[inline]
async unsafe fn download_thread(
    spool: *mut Mutex<TicketSpool>,
    client: *const Client, cobalt_url: *const str,
    pending: PendingDownload!(),
) {
    let result = pending.await;

    let mut lock = match unsafe { spool.as_mut_unchecked() }.lock() {
        Ok(ok) => ok,
        Err(_) => {
            panic!();
        },
    };

    lock.1.push(result); // push results
    let Some(ticket) = lock.0.next() /* next ticket */ else {
        return;
    };
    drop(lock);

    return download_thread(spool, client, cobalt_url, 
        into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked())
    ).await
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
