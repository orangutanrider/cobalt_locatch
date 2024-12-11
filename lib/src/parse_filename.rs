/* 

///! Parsing of filename field from Content-Disposition header
// un-implemented while picker response goes un-implemented

use locatch_depen::*;

use std::str::Chars;

use reqwest::header::HeaderValue;

// Cobalt uses this package for its Coontent-Disposition headers:
// https://www.npmjs.com/package/content-disposition
// They use the setHeader method, and they do not specifiy the fallback option
// fallback defaults to true
// Which means that that the method will automatically generate a ISO-8859-1 filename field, in the case that the system was given a unicode filename.
// So it will contain "filename" and "filename*" in this scenario
// i.e. It is guaranteed to contain the "filename" field, and we don't have to bother with the unicode one

pub fn parse_filename(decoder: &mut Decoder, content_disposition: &HeaderValue) {
	todo!("Picker response handling un-implemented");

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

// https://docs.rs/encoding_rs/latest/encoding_rs/#notable-differences-from-iana-naming
// WINDOWS_1252 is an extension of ISO-8859-1 (also known as Latin 1)
// macro_rules! ISO_8859_1 {() => { WINDOWS_1252 };}
// macro_rules! LATIN_1 {() => { WINDOWS_1252 };}

use encoding_rs::{Decoder, WINDOWS_1252};

#[inline]
pub fn iso_8859_1_decoder() -> Decoder { return WINDOWS_1252.new_decoder(); }

const FIELD_LEN: usize = 9;
const FIELD: [char; FIELD_LEN] = ['f', 'i', 'l', 'e', 'n', 'a', 'm', 'e', '=']; 

// https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6
// https://datatracker.ietf.org/doc/html/rfc9110#name-recipient-requirements
// quoted-string  = DQUOTE *( qdtext / quoted-pair ) DQUOTE
// quoted-pair    = "\" ( HTAB / SP / VCHAR / obs-text )
// qdtext         = HTAB / SP /%x21 / %x23-5B / %x5D-7E / obs-text
// obs-text       = %x80-FF
/*
   The backslash octet ("\") can be used as a single-octet quoting
   mechanism within quoted-string and comment constructs.  Recipients
   that process the value of a quoted-string MUST handle a quoted-pair
   as if it were replaced by the octet following the backslash.
*/

// https://datatracker.ietf.org/doc/html/rfc9110#name-syntax-notation
// HTAB (horizontal tab)
// SP (space)
// VCHAR (any visible US-ASCII character)

// https://learn.microsoft.com/en-us/dotnet/api/microsoft.net.http.headers.headerutilities.unescapeasquotedstring?view=aspnetcore-8.0
// https://github.com/dotnet/aspnetcore/blob/3f1acb59718cadf111a0a796681e3d3509bb3381/src/Http/Headers/src/HeaderUtilities.cs#L616C9-L651C12

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

*/