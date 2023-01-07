use sgx_types::{sgx_status_t, SgxResult};
use crate::keys::VALIDATOR_SET_SEALING_PATH;

use enclave_utils::storage::unseal;
use log::{debug, error};
use tendermint::Hash;
use tendermint_proto::Protobuf;

pub fn get_validator_set_hash() -> SgxResult<Hash> {
    let res = unseal(&VALIDATOR_SET_SEALING_PATH)?;

    // // As of now this is not working because of a difference in behavior between tendermint and tendermint-rs
    // // Ref: https://github.com/informalsystems/tendermint-rs/issues/1255
    let hash = match tendermint::validator::Set::decode(&*res) {
        Ok(vs) => {
            debug!("the validator set hash: {:?}", vs.hash());
            vs.hash()
        }
        Err(e) => {
            error!("error decoding validator set: {:?}", e);
            return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
        },
    };

    Ok(hash)
}
