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
pub async fn into_downloads<D: IntoDownload>(
    client: &Client, cobalt_url: &str, concurrent_download_limit: Option<usize>, 
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    return match concurrent_download_limit {
        Some(limit) => download_with_limit::<D>(client, cobalt_url, limit, tickets).await,
        None => download_unlimited::<D>(client, cobalt_url, tickets).await,
    }
}

#[inline]
async fn download_unlimited<D: IntoDownload>(
    client: &Client, cobalt_url: &str,
    tickets: Vec<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    let mut set = JoinSet::new();
    
    for ticket in tickets.into_iter() {
        set.spawn(unsafe { download_one_shot_thread::<D>(ticket, UnsafeSend(client), UnsafeSend(cobalt_url)) });
    }

    return set.join_all().await;
}

#[inline]
async fn download_with_limit<D: IntoDownload>(
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
        set.spawn(unsafe { download_unravel_thread::<D>(UnsafeSend(&mut spool), UnsafeSend(&mut output), UnsafeSend(client), UnsafeSend(cobalt_url)) });
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
async unsafe fn download_unravel_thread<D: IntoDownload>(
    spool: UnsafeSend<*mut Mutex<std::vec::IntoIter<Ticket>>>,
    output: UnsafeSend<*mut Mutex<Vec<Result<(), LocatchErr>>>>,
    client: UnsafeSend<*const Client>, cobalt_url: UnsafeSend<*const str>,
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
        let result = D::into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;

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
async unsafe fn download_one_shot_thread<D: IntoDownload>(
    ticket: Ticket,
    client: UnsafeSend<*const Client>, cobalt_url: UnsafeSend<*const str>, 
) -> Result<(), LocatchErr> {
    return D::into_download(ticket, client.as_ref_unchecked(), cobalt_url.as_ref_unchecked()).await;
}

pub trait IntoDownload: 'static {
    fn into_download(ticket: Ticket, client: &Client, cobalt_url: &str) -> impl Future<Output = Result<(), LocatchErr>> + Send;
}

pub struct CobaltDownload;
impl IntoDownload for CobaltDownload {
    async fn into_download(ticket: Ticket, client: &Client, cobalt_url: &str) -> Result<(), LocatchErr> {
        // post
        let (response, ticket) = request::<CobaltReqRequest>(client, cobalt_url, ticket).await;

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

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    struct OkDownload; 
    impl IntoDownload for OkDownload {
        async fn into_download(_ticket: Ticket, _client: &Client, _cobalt_url: &str) -> Result<(), LocatchErr> {
            return Ok(())
        }
    }

    struct ErrDownload; 
    impl IntoDownload for ErrDownload {
        async fn into_download(_ticket: Ticket, _client: &Client, _cobalt_url: &str) -> Result<(), LocatchErr> {
            return Err(LocatchErr::Empty)
        }
    }

    fn test_ticket() -> Ticket {
        Ticket {
            url: "empty".to_owned(),
            filename: Some("test_ticket".to_owned()),
            ..Default::default()
        }
    }
    
    const TEST_TICKETS_SIZE: usize = 10;
    fn test_tickets() -> Vec<Ticket> {
        let mut vec = Vec::with_capacity(TEST_TICKETS_SIZE);
        for _ in 0..TEST_TICKETS_SIZE {
            vec.push(test_ticket());
        }

        return vec;
    }

    #[tokio::test]
    async fn ok_download_unlimited_test() {
        let client = &Client::new();
        let cobalt_url: &str = "empty";

        let results = download_unlimited::<OkDownload>(client, cobalt_url, test_tickets()).await;

        assert_eq!(results.len(), TEST_TICKETS_SIZE);
        for result in results.into_iter() {
            match result {
                Ok(_) => {/* Do nothing */},
                Err(err) => {
                    panic!("{}", err);
                },
            }
        }
    }

    #[tokio::test]
    async fn ok_download_limited_test() {
        let client = &Client::new();
        let cobalt_url: &str = "empty";

        const LIMIT: usize = 2;
        let results = download_with_limit::<OkDownload>(client, cobalt_url, LIMIT, test_tickets()).await;

        assert_eq!(results.len(), TEST_TICKETS_SIZE);
        for result in results.into_iter() {
            match result {
                Ok(_) => {/* Do nothing */},
                Err(err) => {
                    panic!("{}", err);
                },
            }
        }
    }

    #[tokio::test]
    async fn err_download_unlimited_test() {
        let client = &Client::new();
        let cobalt_url: &str = "empty";

        let results = download_unlimited::<ErrDownload>(client, cobalt_url, test_tickets()).await;

        assert_eq!(results.len(), TEST_TICKETS_SIZE);
        for result in results.into_iter() {
            match result {
                Ok(_) => { panic!("an Ok() value was returned") }
                Err(err) => {
                    match err {
                        LocatchErr::Io(err) => { panic!("an unexpected Io err was returned: {}", err) },
                        LocatchErr::Json(err) => { panic!("an unexpected Json err was returned: {}", err) },
                        LocatchErr::Req(err) => { panic!("an unexpected Req err was returned: {}", err) },
                        LocatchErr::Empty => {/* Do nothing */},
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn err_download_limited_test() {
        let client = &Client::new();
        let cobalt_url: &str = "empty";

        const LIMIT: usize = 2;
        let results = download_with_limit::<ErrDownload>(client, cobalt_url, LIMIT, test_tickets()).await;

        assert_eq!(results.len(), TEST_TICKETS_SIZE);
        for result in results.into_iter() {
            match result {
                Ok(_) => { panic!("an Ok() value was returned") }
                Err(err) => {
                    match err {
                        LocatchErr::Io(err) => { panic!("an unexpected Io err was returned: {}", err) },
                        LocatchErr::Json(err) => { panic!("an unexpected Json err was returned: {}", err) },
                        LocatchErr::Req(err) => { panic!("an unexpected Req err was returned: {}", err) },
                        LocatchErr::Empty => {/* Do nothing */},
                    }
                }
            }
        }
    }
}