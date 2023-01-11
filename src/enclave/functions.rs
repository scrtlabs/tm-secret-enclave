use crate::enclave::consts::ENCLAVE_FILE_NAME;
use crate::enclave::enclave_api::{ecall_generate_random, ecall_health_check, ecall_submit_validator_set, ecall_validate_random};
use crate::enclave::init::init_enclave;
use crate::Error;
use sgx_types::{sgx_status_t, SgxResult};

pub fn health_check() -> Result<Vec<u8>, Error> {
    let enclave = init_enclave(ENCLAVE_FILE_NAME).unwrap();

    let eid = enclave.geteid();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let status = unsafe { ecall_health_check(eid, &mut retval) };

    if status != sgx_status_t::SGX_SUCCESS {
        println!("could not generate attestation report");
        panic!("omg");
    }

    if retval != sgx_status_t::SGX_SUCCESS {
        println!("could not generate attestation report");
        panic!("omg");
    }

    let result: u64 = 42;

    return Ok(result.to_be_bytes().to_vec());
}

pub fn random_number(block_hash: &[u8], height: u64) -> Result<Vec<u8>, crate::Error> {
    let enclave =
        init_enclave(ENCLAVE_FILE_NAME).map_err(|_| Error::enclave_err("sgx not available"))?;

    let eid = enclave.geteid();
    let mut retval = sgx_status_t::SGX_SUCCESS;

    let mut random = [0u8; 48];
    let mut proof = [0u8; 32];

    let status = unsafe { ecall_generate_random(
        eid,
        &mut retval,
        block_hash.as_ptr(),
        block_hash.len() as u32,
        height,
        &mut random,
        &mut proof,
    ) };

    if retval != sgx_status_t::SGX_SUCCESS {
        return Err(Error::RandomGeneration { msg: "retval unexpected".to_string() });
    }

    if status != sgx_status_t::SGX_SUCCESS {
        return Err(Error::RandomGeneration { msg: "status unexpected".to_string() });
    }

    let mut return_val = vec![];
    return_val.extend_from_slice(&random);
    return_val.extend_from_slice(&proof);
    return Ok(return_val);
}

pub fn next_validator_set(val_set: &[u8], height: u64) -> SgxResult<()> {
    let enclave = init_enclave(ENCLAVE_FILE_NAME)?;

    let eid = enclave.geteid();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let status = unsafe {
        ecall_submit_validator_set(eid, &mut retval, val_set.as_ptr(), val_set.len() as u32, height)
    };

    if status != sgx_status_t::SGX_SUCCESS {
        return Err(status);
    }

    if retval != sgx_status_t::SGX_SUCCESS {
        return Err(retval);
    }

    return Ok(());
}
//
pub fn enclave_validate_random(random: &[u8], proof: &[u8], block_hash: &[u8], height: u64) -> SgxResult<()> {
    let enclave = init_enclave(ENCLAVE_FILE_NAME)?;

    let eid = enclave.geteid();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let status = unsafe {
        ecall_validate_random(
            eid,
            &mut retval,
            random.as_ptr(),
            random.len() as u32,
            proof.as_ptr(),
            proof.len() as u32,
            block_hash.as_ptr(),
            block_hash.len() as u32,
            height
        )
    };

    if status != sgx_status_t::SGX_SUCCESS {
        return Err(status);
    }

    if retval != sgx_status_t::SGX_SUCCESS {
        return Err(retval);
    }

    return Ok(());
}
