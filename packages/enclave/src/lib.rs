mod keys;
pub mod storage;
pub mod error;
mod validator_set;

// Trick to get the IDE to use sgx_tstd even when it doesn't know we're targeting SGX
#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

extern crate sgx_trts;
extern crate sgx_types;
extern crate core;
// extern crate sgx_rand;
//
// use ctor::*;
// use enclave_utils::logger::get_log_level;

use enclave_crypto::{SIVEncryptable, sha_256};
use enclave_utils::logger::get_log_level;
use std::{slice};

use sgx_trts::trts::rsgx_read_rand;
use sgx_types::sgx_status_t;
use crate::keys::{IRS, REK};

use log::{error, info};
use tendermint::Hash;
use tendermint::Hash::Sha256;
use crate::validator_set::get_validator_set_hash;

use ctor::ctor;
use enclave_utils::validator_set::ValidatorSetForHeight;

use enclave_utils::{validate_const_ptr, validate_input_length};

const MAX_VARIABLE_LENGTH: u32 = 100_000;

#[ctor]
fn init_logger() {
    let default_log_level = log::Level::Debug;
    simple_logger::init_with_level(get_log_level(default_log_level)).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn ecall_health_check() -> sgx_status_t {
    return sgx_status_t::SGX_SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn ecall_generate_random(
    block_hash: *const u8,
    block_hash_len: u32,
    height: u64,
    random: &mut [u8; 48],
    proof: &mut [u8; 32]
) -> sgx_status_t {

    validate_const_ptr!(block_hash, block_hash_len as usize, sgx_status_t::SGX_ERROR_INVALID_PARAMETER);

    if block_hash_len != 32 {
        error!("block hash bad length");
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
    }
    let block_hash_slice = slice::from_raw_parts(block_hash, block_hash_len as usize);


    let mut rand_buf: [u8; 32] = [0; 32];

    if let Err(_e) = rsgx_read_rand(&mut rand_buf) {
        error!("Error generating random value");
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
    };

    let validator_set_hash = match get_validator_set_hash().unwrap_or_default() {
        Sha256(hash) => hash,
        Hash::None => {
            error!("Got invalid validator set");
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        }
    };

    // todo: add entropy detection

    let encrypted: Vec<u8> = if let Ok(res) = REK.encrypt_siv(&rand_buf, Some(vec![validator_set_hash.as_slice()].as_slice())) {
        res
    } else {
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
    };

    random.copy_from_slice(encrypted.as_slice());

    // proof is an encrypted value that allows enclaves to validate that the encrypted value was created for
    // this specific height & block. This allows for replay protection.
    // optional improvement: Add public key signatures to be able to validate this outside the enclave
    let proof_computed = create_proof(height, encrypted.as_slice(), block_hash_slice);
    proof.copy_from_slice(proof_computed.as_slice());

    // debug!("Calculated proof: {:?}", proof_computed);

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn ecall_submit_validator_set(
    val_set: *const u8,
    val_set_len: u32,
    height: u64
) -> sgx_status_t {

    validate_input_length!(val_set_len, "validator set length", MAX_VARIABLE_LENGTH);
    validate_const_ptr!(val_set, val_set_len as usize, sgx_status_t::SGX_ERROR_INVALID_PARAMETER);

    let val_set_slice = slice::from_raw_parts(val_set, val_set_len as usize);

    let val_set = ValidatorSetForHeight {
        height,
        validator_set: val_set_slice.to_vec()
    };

    let res = val_set.seal();
    if res.is_err() {
        return sgx_status_t::SGX_ERROR_ENCLAVE_FILE_ACCESS;
    }

    sgx_status_t::SGX_SUCCESS
}

fn create_proof(height: u64, random: &[u8], block_hash: &[u8]) -> [u8; 32] {

    let mut data = vec![];
    data.extend_from_slice(&height.to_be_bytes());
    data.extend_from_slice(random);
    data.extend_from_slice(block_hash);
    data.extend_from_slice(IRS.get());

    sha_256(data.as_slice())
}

#[no_mangle]
pub unsafe extern "C" fn ecall_validate_random(
    random: *const u8,
    random_len: u32,
    proof: *const u8,
    proof_len: u32,
    block_hash: *const u8,
    block_hash_len: u32,
    height: u64,
) -> sgx_status_t {

    validate_input_length!(random_len, "proof", MAX_VARIABLE_LENGTH);
    validate_input_length!(proof_len, "encrypted random", MAX_VARIABLE_LENGTH);
    if block_hash_len != 32 {
        error!("block hash bad length");
        return sgx_status_t::SGX_ERROR_UNEXPECTED;
    }

    validate_const_ptr!(random, random_len as usize, sgx_status_t::SGX_ERROR_INVALID_PARAMETER);
    validate_const_ptr!(proof, proof_len as usize, sgx_status_t::SGX_ERROR_INVALID_PARAMETER);
    validate_const_ptr!(block_hash, block_hash_len as usize, sgx_status_t::SGX_ERROR_INVALID_PARAMETER);

    let random_slice = slice::from_raw_parts(random, random_len as usize);
    let proof_slice = slice::from_raw_parts(proof, proof_len as usize);
    let block_hash_slice = slice::from_raw_parts(block_hash, block_hash_len as usize);

    let calculated_proof = create_proof(height, random_slice, block_hash_slice);

    if &calculated_proof != proof_slice {
        return sgx_status_t::SGX_ERROR_INVALID_SIGNATURE;
    }

    sgx_status_t::SGX_SUCCESS
}
