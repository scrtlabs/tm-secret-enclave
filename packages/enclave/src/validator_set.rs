use sgx_types::{sgx_status_t, SgxResult};

use enclave_utils::validator_set::ValidatorSetForHeight;

use log::{debug, error};
use tendermint::Hash;
use tendermint_proto::Protobuf;
use tendermint_proto::v0_38::types::ValidatorSet as RawValidatorSet;

pub fn get_validator_set_hash() -> SgxResult<Hash> {
    let res = ValidatorSetForHeight::unseal()?; //unseal(&VALIDATOR_SET_SEALING_PATH)?;

    let hash = match <tendermint::validator::Set as Protobuf<RawValidatorSet>>::decode(&*(res.validator_set)) {
        Ok(vs) => {
            debug!("decoded validator set hash: {:?}", vs.hash());
            vs.hash()
        }
        Err(e) => {
            error!("error decoding validator set: {:?}", e);
            return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
        },
    };

    Ok(hash)
}
