use crate::*;

use std::str::Chars;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use reqwest::{
    header::{
        HeaderValue,
        HeaderMap,
        CONTENT_DISPOSITION,
    }, 
    Client, 
    Response,
};


pub enum DownloadError {
    FileError(IoError),
    ReqwestError(ReqError),
}

pub async fn download(client: &Client, url: &str, filename: &str) -> Result<(), DownloadError> {
    let request = client.get(url).send();
    let file = tokio::fs::File::create(filename);

    let mut file = match file.await {
        Ok(ok) => ok,
        Err(err) => return Err(DownloadError::FileError(err)),
    };

    let response = match request.await {
        Ok(ok) => ok,
        Err(err) => return Err(DownloadError::ReqwestError(err)),
    };

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(ok) => {
                match file.write_all(&ok).await {
                    Ok(_) => {/* Do nothing */},
                    Err(err) => return Err(DownloadError::FileError(err)),
                };
            },
            Err(err) => return Err(DownloadError::ReqwestError(err)),
        }
    }

    match file.flush().await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(DownloadError::FileError(err)),
    }
}

// https://docs.rs/encoding_rs/latest/encoding_rs/#notable-differences-from-iana-naming
// WINDOWS_1252 is an extension of ISO-8859-1 (also known as Latin 1)
// macro_rules! ISO_8859_1 {() => { WINDOWS_1252 };}
// macro_rules! LATIN_1 {() => { WINDOWS_1252 };}

use encoding_rs::{Decoder, WINDOWS_1252};

#[inline]
pub fn iso_8859_1_decoder() -> Decoder { return WINDOWS_1252.new_decoder(); }

// Cobalt uses this package for its Coontent-Disposition headers:
// https://www.npmjs.com/package/content-disposition
// They use the setHeader method, and they do not specifiy the fallback option
// fallback defaults to true
// Which means that that the method will automatically generate a ISO-8859-1 filename field, in the case that the system was given a unicode filename.
// So it will contain "filename" and "filename*" in this scenario
// i.e. It is guaranteed to contain the "filename" field, and we don't have to bother with the unicode one

pub fn get_filename(decoder: &mut Decoder, content_disposition: &HeaderValue) {
    let bytes = content_disposition.as_bytes();

    // Potential for optimization here;
    // Pre-allocation or using variable-length-arrays for stack allocation is theoretically prefferble.
    // This function is run for every picker response, and their contained downloads, which can be processed in parralel; a ton of heap allocation system calls during that doesn't sound ideal.
    let mut contents = String::with_capacity(bytes.len());

    // Potentially optimizable by only decoding the bytes we care about rather than the header's full contents.
    // Semantics of function can be inferred from the documentation page, tooltip is minimal.
    // https://docs.rs/encoding_rs/latest/encoding_rs/struct.Decoder.html#
    let (_, _, _) = decoder.decode_to_string(bytes, &mut contents, true);
    // Continues until the end of the string, replacing malformed characters with a REPLACEMENT CHARACTER

    let contents = contents.chars();
}

// filename can be inserted like so filename="filename="
// The system has to account for this.

// Start by continuing until ';'
// Ignore whitespace
// Expect "filename=" or continue 
// While continuing
    // If a " is detected do A
    // otherwise do B
// A
    // Continue until "
    // If \ detected, remember that
    // If " detected after a \ continue
    // Expect ; after exiting
// B
    // Continue until ;
// Ignore whitespace
// Expect "filename=" or repeat 


const FILENAME: [char; 9] = ['f', 'i', 'l', 'e', 'n', 'a', 'm', 'e', '=']; 
fn combo_until_filename(mut iter: Chars<'_>) {

}

//pub fn test(headers: &HeaderMap) {
//    let content_disposition = headers.get(CONTENT_DISPOSITION);
//}
