use locatch_macro::*;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::join;
use reqwest::Client;

pub async fn download(client: &Client, url: &str, filename: &str) -> Result<(), LocatchErr> {
    let file = tokio::fs::File::create(filename);
    let pending = client.get(url).send();

    let (file, pending) = join!(file, pending);
    
    let mut file = match file {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Io(err)),
    };

    let pending = match pending {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Req(err)),
    };

    let mut stream = pending.bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(ok) => {
                match file.write_all(&ok).await {
                    Ok(_) => {/* Do nothing */},
                    Err(err) => return Err(LocatchErr::Io(err)),
                };
            },
            Err(err) => return Err(LocatchErr::Req(err)),
        }
    }

    match file.flush().await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(LocatchErr::Io(err)),
    }
}