// This is an experiment.
// At the time of creation it is the same as download, but it decouples into_download by making it a closure instead of a generic trait.
// Could maybe create an implementation that allows for both.
// No need for that right now though.

use reqwest::Client;

use tokio::task::*;
use std::sync::Mutex;
use std::future::Future;

use locatch_macro::*;
use locatch_macro::unsafe_lib::UnsafeSend;
use locatch_lib::*;

use crate::serial_input::*;
use crate::request;

#[inline]
async fn into_downloads(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>,
    into: impl IntoDownload
) -> Vec<Result<(), LocatchErr>> {
    return match concurrent_download_limit {
        Some(limit) => download_with_limit(client, cobalt_url, limit, tickets, into).await,
        None => download_unlimited(client, cobalt_url, tickets, into).await,
    }
}

#[inline]
async fn download_unlimited(
    client: &Client, cobalt_url: &str,
    tickets: Vec<Ticket>,
    into: impl IntoDownload,
) -> Vec<Result<(), LocatchErr>> {
    let mut set = JoinSet::new();
    
    for ticket in tickets.into_iter() {
        set.spawn(unsafe { download_one_shot_thread(ticket, UnsafeSend(client), UnsafeSend(cobalt_url), into) });
    }

    return set.join_all().await;
}

#[inline]
async fn download_with_limit(
    client: &Client, cobalt_url: &str, limit: usize, 
    tickets: Vec<Ticket>,
    into: impl IntoDownload
) -> Vec<Result<(), LocatchErr>> { 
    let len = tickets.len();
    let tickets = tickets.into_iter();
    let output = Vec::with_capacity(len);

    let mut set = JoinSet::new();
    let mut spool = Mutex::new(tickets);
    let mut output = Mutex::new(output);

    for _ in 0..limit {
        set.spawn(unsafe { download_unravel_thread(UnsafeSend(&mut spool), UnsafeSend(&mut output), UnsafeSend(client), UnsafeSend(cobalt_url), into) });
    }

    set.join_all().await;

    return match output.into_inner() {
        Ok(ok) => ok,
        Err(_) => panic!(),
    };
}

// An Arc isn't needed because the thread is joined before the scope ends.
/// It will process tickets from the spool, adding them to the output, until no tickets are left.  
/// Unsafe: It is expected that the thread is joined before any of the inputs are dropped.
#[inline]
async unsafe fn download_unravel_thread(
    spool: UnsafeSend<*mut Mutex<std::vec::IntoIter<Ticket>>>,
    output: UnsafeSend<*mut Mutex<Vec<Result<(), LocatchErr>>>>,
    client: UnsafeSend<*const Client>, cobalt_url: UnsafeSend<*const str>,
    into: impl IntoDownload
) {
    loop {
        let ticket = {
            let mut spool_lock = match unsafe { spool.as_mut_unchecked() }.lock() {
                Ok(ok) => ok,
                Err(_) => {
                    panic!();
                },
            };

            let Some(ticket) = spool_lock.next() else {
                return;
            };

            ticket
        };
        
        // await 
        let result = into.into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;

        {
            let mut output_lock = match unsafe { output.as_mut_unchecked() }.lock() {
                Ok(ok) => ok,
                Err(_) => {
                    panic!();
                },
            };

            output_lock.push(result);
        }

        yield_now().await; // drop lock before await; Yield to other threads.
    }
}

// An Arc isn't needed because the thread is joined before the scope ends.
/// Unsafe: It is expected that the thread is joined before any of the inputs are dropped.
#[inline]
async unsafe fn download_one_shot_thread(
    ticket: Ticket,
    client: UnsafeSend<*const Client>, cobalt_url: UnsafeSend<*const str>, 
    into: impl IntoDownload
) -> Result<(), LocatchErr> {
    return into.into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;
}

trait IntoDownload: Copy + Send +'static {
    fn into_download(&self, ticket: Ticket, client: &Client, cobalt_url: &str) -> impl Future<Output = Result<(), LocatchErr>> + Send;
}

impl<T: Fn(Ticket, &Client, &str) -> Result<(), LocatchErr> + Copy + Send + Sync + 'static> IntoDownload for T {
    async fn into_download(&self, ticket: Ticket, client: &Client, cobalt_url: &str) -> Result<(), LocatchErr> {
        self(ticket, client, cobalt_url)
    }
}

async fn into_cobalt_download(ticket: Ticket, client: &Client, cobalt_url: &str) -> Result<(), LocatchErr> {
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
