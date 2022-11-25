mod error;
mod logger;
mod memory;
/// cbindgen:ignore
mod enclave;

use logger::get_log_level;
pub use memory::{free_rust, Buffer};

use std::convert::TryInto;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::str::from_utf8;

use crate::error::{clear_error, handle_c_error, handle_c_error_default, set_error, Error};

use ctor::ctor;
use log::*;
use crate::enclave::functions::health_check;

#[ctor]
fn init_logger() {
    let default_log_level = log::Level::Info;
    simple_logger::init_with_level(get_log_level(default_log_level)).unwrap();
}

#[no_mangle]
pub extern "C" fn get_health_check(err: Option<&mut Buffer>) -> Buffer {

    match health_check() {
        Err(e) => {
            set_error(Error::enclave_err(e.to_string()), err);
            Buffer::default()
        }
        Ok(res) => {
            clear_error();
            Buffer::from_vec(format!("{:?}", res).into_bytes())
        }
    }
}
