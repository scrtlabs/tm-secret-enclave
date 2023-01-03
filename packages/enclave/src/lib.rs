// Trick to get the IDE to use sgx_tstd even when it doesn't know we're targeting SGX
#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

extern crate sgx_trts;
extern crate sgx_types;
// extern crate sgx_rand;
//
// use ctor::*;
// use enclave_utils::logger::get_log_level;

use std::{convert::TryFrom, slice};

use sgx_trts::trts::rsgx_read_rand;
use sgx_types::sgx_status_t;
use tendermint::validator::Set;
use tendermint_proto::Protobuf;

// #[cfg(feature = "production")]
// #[ctor]
// fn init_logger() {
//     let default_log_level = log::Level::Warn;
//     simple_logger::init_with_level(get_log_level(default_log_level)).unwrap();
// }
//
// #[cfg(all(not(feature = "production"), not(feature = "test")))]
// #[ctor]
// fn init_logger() {
//     let default_log_level = log::Level::Trace;
//     simple_logger::init_with_level(get_log_level(default_log_level)).unwrap();
// }

/// # Safety
/// Always use protection
#[no_mangle]
pub unsafe extern "C" fn ecall_health_check() -> sgx_status_t {
    return sgx_status_t::SGX_SUCCESS;
}

/// # Safety
/// Always use protection
#[no_mangle]
pub unsafe extern "C" fn ecall_generate_random() -> u64 {
    println!("AFTER");
    let mut rand_buf: [u8; 8] = [0; 8];

    match rsgx_read_rand(&mut rand_buf) {
        Ok(_) => u64::from_be_bytes(rand_buf),
        Err(_) => 0,
    }
}

/// # Safety
/// Always use protection
#[no_mangle]
pub unsafe extern "C" fn ecall_submit_validator_set(
    val_set: *const u8,
    val_set_len: u32,
) -> sgx_status_t {
    let val_set_slice = slice::from_raw_parts(val_set, val_set_len as usize);

    // As of now this is not working because of a difference in behavior between tendermint and tendermint-rs
    // Ref: https://github.com/informalsystems/tendermint-rs/issues/1255
    match Set::decode(val_set_slice) {
        Ok(vs) => {
            println!("this is a validator set from within the enclave: {:?}", vs);
            println!("the validator set hash: {:?}", vs.hash());
        }
        Err(e) => println!("error decoding validator set: {:?}", e),
    };

    sgx_status_t::SGX_SUCCESS
}
