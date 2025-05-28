use locatch_macro::*;
use locatch_macro::unsafe_lib::UnsafeSend;

use std::path::PathBuf;
use tokio::fs;

/// Unsafe: It is expected that cli will not be dropped or mutated until the thread has joined.
pub(super) async unsafe fn required_reception_thread<'de, T: FromJson<'de, String>> (cli: UnsafeSend<*const PathBuf>) -> Result<T, LocatchErr> {
    let serial = match fs::read_to_string(cli.as_ref_unchecked()).await {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Io(err)),
    };

    let deserial = match T::from_json(serial) {
        Ok(ok) => ok,
        Err(err) => return Err(LocatchErr::Json(err)),
    };

    return Ok(deserial);
}
