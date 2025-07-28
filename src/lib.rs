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

/// The default implicit hash: SHA-256 hash of an empty slice.
/// (e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855)
const DEFAULT_IMPLICIT_HASH: [u8; 32] = [
    0xe3, 0xb0, 0xc4, 0x42,
    0x98, 0xfc, 0x1c, 0x14,
    0x9a, 0xfb, 0xf4, 0xc8,
    0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4,
    0x64, 0x9b, 0x93, 0x4c,
    0xa4, 0x95, 0x99, 0x1b,
    0x78, 0x52, 0xb8, 0x55,
];

/// Global storage for the implicit hash protected by a Mutex.
static IMPLICIT_HASH: Mutex<[u8; 32]> = Mutex::new(DEFAULT_IMPLICIT_HASH);

/// ECALL: Sets the implicit hash in the enclave.
/// Expects a Buffer containing exactly 32 bytes.
/// On error, sets the error via the provided `err` parameter.
#[no_mangle]
pub extern "C" fn set_implicit_hash(hash: Buffer, err: Option<&mut Buffer>) {
    let hash_slice = match unsafe { hash.read() } {
        None => {
            set_error(Error::empty_arg("implicit_hash"), err);
            return;
        }
        Some(s) => s,
    };

    if hash_slice.len() != 32 {
        set_error(Error::enclave_err("Invalid implicit_hash length: expected 32 bytes"), err);
        return;
    }

    match IMPLICIT_HASH.lock() {
        Ok(mut guard) => {
            // Copy the provided 32 bytes into the global storage.
            for (i, &byte) in hash_slice.iter().enumerate() {
                guard[i] = byte;
            }
        }
        Err(_) => {
            set_error(Error::enclave_err("Failed to lock implicit hash storage"), err);
            return;
        }
    }

    clear_error();
}

/// ECALL: Retrieves the stored implicit hash from the enclave.
/// Returns a Buffer containing 32 bytes.
/// If an error occurs, sets the error via the provided `err` parameter and returns a default Buffer.
#[no_mangle]
pub extern "C" fn get_implicit_hash(err: Option<&mut Buffer>) -> Buffer {
    let guard = match IMPLICIT_HASH.lock() {
        Ok(g) => g,
        Err(_) => {
            set_error(Error::enclave_err("Failed to lock implicit hash storage"), err);
            return Buffer::default();
        }
    };

    let hash_vec = guard.to_vec();
    clear_error();
    Buffer::from_vec(hash_vec)
}