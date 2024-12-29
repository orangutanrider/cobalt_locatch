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
fn pin_iter_mut<T>(slice: Pin<&mut [T]>) -> impl Iterator<Item = Pin<&mut T>> {
    return unsafe { slice.get_unchecked_mut() }
        .iter_mut()
        .map(|t| unsafe { Pin::new_unchecked(t) })
}

#[inline]
fn pin_access_at<T>(slice: Pin<&mut [T]>, i: usize) -> Pin<&mut T> {
    return unsafe { Pin::new_unchecked(
        &mut slice.get_unchecked_mut()[i] 
    ) }
}

// #[inline]
// fn pin_set_at<T>(slice: Pin<&mut [T]>, i: usize, val: T) {
//     unsafe {
//         slice.get_unchecked_mut()[i] = val;
//     }
// }

// #[inline]
// fn poll_vec_set_at(slice: Pin<&mut [PendingDownload!()]>, i: usize, val: PendingDownload!()) {
//     unsafe {
//         slice.get_unchecked_mut()[i] = val;
//     }
// }

// #[inline]
// fn pin_set_at(slice: Pin<&mut [impl Future<Output = Result<(), LocatchErr>>]>, i: usize, val: impl Future<Output = Result<(), LocatchErr>>) {
//     unsafe {
//         slice.get_unchecked_mut()[i] = val;
//     }
// }

// #[inline]
// fn set_at(slice: &mut [impl Future<Output = Result<(), LocatchErr>>], i: usize, val: impl Future<Output = Result<(), LocatchErr>>) {
//     //let entry = &mut slice[i];
//     //let val = val;
// }

// fn set_element(slice: &mut [impl TestTrait], val: impl TestTrait, index: usize) {
//     slice[index] = val;
// }

// fn set_element<T: TestTrait>(slice: &mut [T], val: T, index: usize) {
//     slice[index] = val;
// }

// fn set_element<T: Future<Output = Result<(), LocatchErr>>>(slice: &mut [T], val: T, index: usize) {
//     slice[index] = val;
//}

// fn set_element<
//     T: Future<Output = Result<(), LocatchErr>>
// > (
//     slice: Pin<&mut [T]>, val: T, index: usize
// ) {
//     unsafe {
//         slice.get_unchecked_mut()[index] = val;   
//     }
// }

// fn set_element(
//     slice: Pin<&mut [impl Future<Output = Result<(), LocatchErr>>]>, 
//     val: impl Future<Output = Result<(), LocatchErr>>, 
//     index: usize
// ) {
//     unsafe {
//         slice.get_unchecked_mut()[index] = val;   
//     }
// }

trait Foo { }

fn set_element(
    slice: &mut [impl Foo], val: impl Foo, index: usize
) {
    slice[index] = val;
}

// struct Test{
//     data: impl Future<Output = Result<(), LocatchErr>>
// }

// #[inline]
// fn set_at(slice: &mut [impl Future<Output = Result<(), LocatchErr>>], i: usize, val: impl Future<Output = Result<(), LocatchErr>>) {
//     slice[i] = val;
// }

// #[inline]
// fn pending_download_set_at(
//     slice: Pin<&mut [impl Future<Output = Result<(), LocatchErr>>]>, 
//     i: usize, 
//     val: impl Future<Output = Result<(), LocatchErr>>
// ) {
//     unsafe {
//         //slice.get_unchecked_mut()[i] = val;
//         let pin = Pin::new_unchecked(slice.get_unchecked_mut().get_mut(i).unwrap());
//         Pin::new_unchecked(slice.get_unchecked_mut().get_mut(i).unwrap()).set(val);
//     }
// }

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
    client: &Client, cobalt_url: &str,
    mut output_vec: Vec<Result<(), LocatchErr>>, mut open_vec: Vec<usize>, mut poll_vec: Pin<&mut [impl Future<Output = Result<(), LocatchErr>>]>,
    mut tickets: std::vec::IntoIter<Ticket>
) -> Vec<Result<(), LocatchErr>> {
    loop {
        // poll
        let mut i: usize = 0;
        for pending in pin_iter_mut(poll_vec.as_mut()) {
            match pending.poll(cx) {
                Poll::Ready(val) => {
                    output_vec.push(val);
                    open_vec.push(i)
                },
                Poll::Pending => {/* Do Nothing */},
            }

            i = i + 1;
        }

        // refill
        for index in open_vec.iter() {
            let Some(ticket) = tickets.next() else {
                break;
            };


            let new = into_download(ticket, client, cobalt_url);
            //access_pin_mut::<impl Future<Output = Result<(), LocatchErr>>>(poll_vec, *index).set(into_download(ticket, client, cobalt_url));
            //access_pin_mut(poll_vec, *index).set(new);
            //pin_set_at(poll_vec, *index, new);
            set_element::<Future<Output = Result<(), LocatchErr>>>(poll_vec, new, *index);
        }

        todo!()
    }

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
