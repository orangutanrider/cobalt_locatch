use locatch_macro::*;

use std::path::PathBuf;
use tokio::fs;

/* 
pub(super) async fn required_reception<'de, T: FromJson<'de, String>> (cli: &PathBuf) -> Result<T, LocatchErr> {
    let serial = match fs::read_to_string(cli).await {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Io(err)),
    };

    let deserial = match T::from_json(serial) {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Json(err)),
    };

    return Ok(deserial);
}
*/

pub(super) async fn fallback_reception<'de, T: FromJson<'de, String>> (cli: &Option<PathBuf>, fallback: &str) -> Result<T, LocatchErr> {
    let serial = match cli {
        Some(val) => fs::read_to_string(val).await,
        None => fs::read_to_string(fallback).await,
    };

    let serial = match serial {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Io(err)),
    };
    
    let deserial = match T::from_json(serial) {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Json(err)),
    };

    return Ok(deserial);
}

pub(super) async fn optional_reception<'de, T: FromJson<'de, String>>(cli: &Option<PathBuf>) -> Result<Option<T>, LocatchErr> {
    let serial = match cli {
        Some(val) => fs::read_to_string(val).await,
        None => return Ok(None),
    };

    let serial = match serial {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Io(err)),
    };
    
    let deserial = match T::from_json(serial) {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Json(err)),
    };

    return Ok(Some(deserial));
}