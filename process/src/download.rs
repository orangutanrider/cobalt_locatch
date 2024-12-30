use reqwest::Client;

use tokio::task::*;
use std::sync::Mutex;

use locatch_macro::*;
use locatch_lib::*;

use crate::serial_input::*;
use crate::req::*;

#[inline]
pub async fn into_downloads(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    return match concurrent_download_limit {
        Some(limit) => download_with_limit(client, cobalt_url, limit, tickets).await,
        None => download_unlimited(client, cobalt_url, tickets).await,
    }
}

#[inline]
async fn download_unlimited(
    client: &Client, cobalt_url: &str,
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    let mut set = JoinSet::new();

    for ticket in tickets.into_iter() {
        set.spawn_local(unsafe { download_one_shot_thread(ticket, client, cobalt_url) });
    }

    return set.join_all().await;
}

#[inline]
async fn download_with_limit(
    client: &Client, cobalt_url: &str, limit: usize, 
    tickets: Vec<Ticket>,
) -> Vec<Result<(), LocatchErr>> { 
    let len = tickets.len();
    let tickets = tickets.into_iter();
    let output = Vec::with_capacity(len);

    let mut set = JoinSet::new();
    let mut spool = Mutex::new(tickets);
    let mut output = Mutex::new(output);

    for _ in 0..limit {
        set.spawn_local(unsafe { download_unravel_thread(&mut spool, &mut output, client, cobalt_url) });
    }

    set.join_all().await;

    return match output.into_inner() {
        Ok(ok) => ok,
        Err(_) => panic!(),
    };
}

// An Arc isn't needed because the thread is joined before the scope ends.
/// It will process tickets from the spool, adding them to the output, until no tickets are left.  
/// Unsafe: It is expected that the thread is joined before any of this function's inputs are dropped.
#[inline]
async unsafe fn download_unravel_thread(
    spool: *mut Mutex<std::vec::IntoIter<Ticket>>,
    output: *mut Mutex<Vec<Result<(), LocatchErr>>>,
    client: *const Client, cobalt_url: *const str,
) {
    loop {
        let mut spool_lock = match unsafe { spool.as_mut_unchecked() }.lock() {
            Ok(ok) => ok,
            Err(_) => {
                panic!();
            },
        };

        let Some(ticket) = spool_lock.next() else {
            return;
        };
        drop(spool_lock); // drop lock before await
        
        // await 
        let result = into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;

        let mut output_lock = match unsafe { output.as_mut_unchecked() }.lock() {
            Ok(ok) => ok,
            Err(_) => {
                panic!();
            },
        };

        output_lock.push(result);
        drop(output_lock);

        yield_now().await; // drop lock before await; Yield to other threads.
    }
}

// An Arc isn't needed because the thread is joined before the scope ends.
/// Unsafe: It is expected that the thread is joined before any of this function's inputs are dropped.
#[inline]
async unsafe fn download_one_shot_thread(
    ticket: Ticket,
    client: *const Client, cobalt_url: *const str, 
) -> Result<(), LocatchErr> {
    return into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;
}


#[inline]
async fn into_download(ticket: Ticket, client: &Client, cobalt_url: &str) -> Result<(), LocatchErr> {
    // post
    let (response, ticket) = request(client, cobalt_url, ticket).await;

    let response = match response {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    match response {
        PostResponse::Redirect(tunnel) => return tunnel_download(client, tunnel, ticket).await,
        PostResponse::Tunnel(tunnel) => return tunnel_download(client, tunnel, ticket).await,
        PostResponse::Picker(_picker) => todo!(),
        PostResponse::Error(_error) => todo!(),
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
