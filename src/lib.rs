mod enclave;
mod error;
mod logger;
mod memory;

use crate::enclave::functions::random_number;
use crate::error::{clear_error, set_error, Error};
use ctor::ctor;
use enclave::functions::next_validator_set;
use logger::get_log_level;
use memory::Buffer;
use std::sync::Mutex;

#[ctor]
fn init_logger() {
    let default_log_level = log::Level::Info;
    simple_logger::init_with_level(get_log_level(default_log_level)).unwrap();
}

#[no_mangle]
pub extern "C" fn validate_random(random: Buffer, proof: Buffer, block_hash: Buffer, height: u64) -> bool {
    let random_slice = match unsafe { random.read() } {
        None => {
            //set_error(Error::empty_arg("val_set"), err);
            return false;
        }
        Some(r) => r,
    };
    let proof_slice = match unsafe { proof.read() } {
        None => {
            //set_error(Error::empty_arg("val_set"), err);
            return false;
        }
        Some(r) => r,
    };
    let block_hash_slice = match unsafe { block_hash.read() } {
        None => {
            //set_error(Error::empty_arg("val_set"), err);
            return false;
        }
        Some(r) => r,
    };

    match crate::enclave::functions::enclave_validate_random(random_slice, proof_slice, block_hash_slice, height) {
        Err(_e) => {
            // set_error(Error::enclave_err(e.to_string()), err);
            false
        }
        Ok(_) => true,
    }
}

#[no_mangle]
pub extern "C" fn get_random_number(block_hash: Buffer, height: u64, err: Option<&mut Buffer>) -> Buffer {
    let block_hash_slice = match unsafe { block_hash.read() } {
        None => {
            set_error(Error::empty_arg("block_hash"), err);
            return Buffer::default();
        }
        Some(r) => r,
    };

    match random_number(block_hash_slice, height) {
        Err(e) => {
            set_error(Error::enclave_err(e.to_string()), err);
            Buffer::default()
        }
        Ok(res) => {
            clear_error();
            Buffer::from_vec(res)
        }
    }
}

#[no_mangle]
pub extern "C" fn submit_next_validator_set(val_set: Buffer, height: u64, err: Option<&mut Buffer>) {
    let val_set_slice = match unsafe { val_set.read() } {
        None => {
            set_error(Error::empty_arg("val_set"), err);
            return;
        }
        Some(r) => r,
    };

    match next_validator_set(val_set_slice, height) {
        Err(e) => {
            set_error(Error::enclave_err(e.to_string()), err);
            return;
        }
        Ok(_) => clear_error(),
    }
}

/// Global storage for implicit transactions (serialized as Tendermint Data)
static IMPLICIT_TXS_DATA: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// ECALL: Sets the implicit transactions in the enclave.
#[no_mangle]
pub extern "C" fn set_scheduled_txs(txs_data: Buffer, err: Option<&mut Buffer>) {
    let data_bytes = match unsafe { txs_data.read() } {
        None => {
            set_error(Error::empty_arg("scheduled_transactions"), err);
            return;
        }
        Some(s) => s,
    };

    match IMPLICIT_TXS_DATA.lock() {
        Ok(mut guard) => {
            *guard = data_bytes.to_vec();
        }
        Err(_) => {
            set_error(Error::enclave_err("Failed to lock implicit transactions storage"), err);
            return;
        }
    }

    clear_error();
}

/// ECALL: Retrieves the stored implicit transactions from the enclave.
#[no_mangle]
pub extern "C" fn get_scheduled_txs(err: Option<&mut Buffer>) -> Buffer {
    let guard = match IMPLICIT_TXS_DATA.lock() {
        Ok(g) => g,
        Err(_) => {
            set_error(Error::enclave_err("Failed to lock scheduled transactions storage"), err);
            return Buffer::default();
        }
    };

    let data = guard.clone();
    clear_error();
    Buffer::from_vec(data)
}