mod blocking;
mod blocking_thread;
use blocking::*;
use blocking_thread::*;

// large file input could mean reading that file in chunks and streaming it to the system, instead of halting until the entire file gets read.
// i.e. a non-blocking reception

use locatch_macro::*;
use locatch_macro::unsafe_lib::UnsafeSend;

use crate::cli::*;
use crate::serial_input::{Config, FilenameMacro, List, TicketMacro};

use tokio::join;

pub type RecievedInput = (Config, List);

#[inline]
pub async fn reception(cli: &Cli) -> Result<RecievedInput, LocatchErr> {
    return blocking_reception(cli).await
}

#[inline]
async fn blocking_reception(cli: &Cli) -> Result<RecievedInput, LocatchErr> {
    // potentially large file reception
    let list = tokio::spawn(unsafe { required_reception_thread::<List>(UnsafeSend(&cli.list)) });

    // small file reception
    let config = fallback_reception::<Config>(&cli.config, CONFIG_FALLBACK);
    let filename_macro = optional_reception::<FilenameMacro>(&cli.filename_macro);
    let ticket_macro = optional_reception::<TicketMacro>(&cli.ticket_macro);

    // await
    let (config, filename_macro, ticket_macro) = join!(config, filename_macro, ticket_macro);

    // unwraps
    let config = match config {
        Ok(ok) => ok,
        Err(err) => {
            list.abort();
            return Err(err)
        },
    };
    
    let filename_macro = match filename_macro {
        Ok(ok) => ok,
        Err(err) => {
            list.abort();
            return Err(err)
        },
    };

    let ticket_macro = match ticket_macro {
        Ok(ok) => ok,
        Err(err) => {
            list.abort();
            return Err(err)
        },
    };

    // await
    let mut list = match list.await {
        Ok(ok) => match ok {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        },
        Err(_) => panic!(),
    };

    // apply macros
    match ticket_macro {
        Some(val) => list.apply_ticket_macro(&val),
        None => {/* Do nothing */},
    }

    match filename_macro {
        Some(val) => list.apply_filename_macro(&val),
        None => {/* Do nothing */},
    }

    list.apply_internal_macros();

    // return
    return Ok((config, list))
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::blocking_reception;

    #[tokio::test]
    async fn no_macro_blocking_reception() {
        let cli = crate::Cli{ 
            list: "list.json".into(), 
            config: Some("config.json".into()), 
            output: None, 
            filename_macro: None, 
            ticket_macro: None 
        };

        let list = json!{{
            "tickets": [
                {
                    "url":"https://www.youtube.com/watch?v=foobar",
                    "filename":"foobar"
                },
            ],
        }}.to_string();

        let config = json!{{
            "cobalt_url": "https://foobar.com",
        }}.to_string();

        match tokio::fs::write("list.json", list).await {
            Ok(_) => {/* Do nothing */},
            Err(err) => panic!("{}", err),
        }

        match tokio::fs::write("config.json", config).await {
            Ok(_) => {/* Do nothing */},
            Err(err) => panic!("{}", err),
        }

        match blocking_reception(&cli).await {
            Ok((config, list)) => {
                assert_eq!(config.cobalt_url, "https://foobar.com");
                assert_eq!(list.tickets[0].url, "https://www.youtube.com/watch?v=foobar");
                assert_eq!(list.tickets[0].filename, Some("foobar".to_owned()));
            },
            Err(err) => panic!("{}", err),
        }
    }
}
