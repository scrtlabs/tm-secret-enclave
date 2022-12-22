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
    // todo validate_const_ptr!()

    let val_set_slice = slice::from_raw_parts(val_set, val_set_len as usize);

    // let v = ValidatorSet::from(val_set_slice);
    match Set::decode(val_set_slice) {
        Ok(vs) => println!("YES: {:?}", vs),
        Err(e) => println!("KUS EMEK ERROR: {:?}", e),
    };

    sgx_status_t::SGX_SUCCESS
}

// #[derive(Clone)]
// pub struct Validator {

// }

// #[derive(Clone, Debug)]
// pub struct ValidatorSet {
//     pub validators: Vec<Validator>,
//     pub proposer: Validator,
//     pub total_voting_power: i64,
// }

// impl Protobuf<RawValidatorSet> for ValidatorSet {}

// impl TryFrom<RawValidatorSet> for ValidatorSet {
//     fn try_from(value: RawValidatorSet) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// impl From<RawValidatorSet> for ValidatorSet {}

// impl Protobuf<RawBlockId> for BlockId {}

// // Example implementation of a protobuf struct using Protobuf.
// #[derive(Clone, Debug)]
// pub struct BlockId {
//     hash: String,
//     part_set_header_exists: bool,
// }

// // Domain types MUST have the TryFrom trait to convert from Protobuf messages.
// impl TryFrom<RawBlockId> for BlockId {
//     type Error = &'static str;

//     fn try_from(value: RawBlockId) -> Result<Self, Self::Error> {
//         Ok(BlockId {
//             hash: String::from_utf8(value.hash)
//                 .map_err(|_| "Could not convert vector to string")?,
//             part_set_header_exists: value.part_set_header.is_some(),
//         })
//     }
// }

// // Domain types MUST be able to convert to Protobuf messages without errors using the From trait.
// impl From<BlockId> for RawBlockId {
//     fn from(value: BlockId) -> Self {
//         RawBlockId {
//             hash: value.hash.into_bytes(),
//             part_set_header: match value.part_set_header_exists {
//                 true => Some(RawPartSetHeader {
//                     total: 0,
//                     hash: vec![],
//                 }),
//                 false => None,
//             },
//         }
//     }
// }

// // Do any custom implementation for your type
// impl PartialEq for BlockId {
//     fn eq(&self, other: &Self) -> bool {
//         self.part_set_header_exists == other.part_set_header_exists && self.hash == other.hash
//     }
// }
