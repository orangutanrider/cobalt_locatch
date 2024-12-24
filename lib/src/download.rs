use locatch_macro::*;

use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use reqwest::Client;

pub enum DownloadError {
    FileError(IOError),
    ReqwestError(ReqError),
}
impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DownloadError::FileError(error) => write!(f, "{}", error),
            DownloadError::ReqwestError(error) => write!(f, "{}", error),
        }
    }
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