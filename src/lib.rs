mod enclave;
mod error;
mod logger;
mod memory;

use enclave::functions::next_validator_set;
use logger::get_log_level;
pub use memory::Buffer;

// use std::convert::TryInto;
// use std::panic::{catch_unwind, AssertUnwindSafe};
// use std::str::from_utf8;

use crate::error::{
    clear_error,
    // handle_c_error, handle_c_error_default,
    set_error,
    Error,
};

use ctor::ctor;
// use log::*;
use crate::enclave::functions::{health_check, random_number};

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

#[no_mangle]
pub extern "C" fn get_random_number(err: Option<&mut Buffer>) -> Buffer {
    match random_number() {
        Err(e) => {
            set_error(Error::enclave_err(e.to_string()), err);
            Buffer::default()
        }
        Ok(res) => {
            clear_error();
            Buffer::from_vec(res.to_be_bytes().to_vec())
        }
    }
}

#[no_mangle]
pub extern "C" fn submit_next_validator_set(val_set: Buffer, err: Option<&mut Buffer>) {
    let val_set_slice = match unsafe { val_set.read() } {
        None => {
            set_error(Error::empty_arg("val_set"), err);
            return;
        }
        Some(r) => r,
    };

    match next_validator_set(val_set_slice) {
        Err(e) => {
            set_error(Error::enclave_err(e.to_string()), err);
            return;
        }
        Ok(_) => clear_error(),
    }
}
