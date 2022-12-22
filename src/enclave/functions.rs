use crate::enclave::consts::ENCLAVE_FILE_NAME;
use crate::enclave::enclave_api::{
    ecall_generate_random, ecall_health_check, ecall_submit_validator_set,
};
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

pub fn random_number() -> Result<u64, crate::Error> {
    let enclave =
        init_enclave(ENCLAVE_FILE_NAME).map_err(|_| Error::enclave_err("sgx not available"))?;

    let eid = enclave.geteid();
    let mut retval: u64 = 0;
    println!("BEFORE");
    let _status = unsafe { ecall_generate_random(eid, &mut retval) };

    return Ok(retval);
}

pub fn next_validator_set(val_set: &[u8]) -> SgxResult<()> {
    let enclave = init_enclave(ENCLAVE_FILE_NAME)?; //.map_err(|_| Error::enclave_err("sgx not available"))?;

    let eid = enclave.geteid();
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let status = unsafe {
        ecall_submit_validator_set(eid, &mut retval, val_set.as_ptr(), val_set.len() as u32)
    };

    if status != sgx_status_t::SGX_SUCCESS {
        return Err(status);
    }

    if retval != sgx_status_t::SGX_SUCCESS {
        return Err(retval);
    }

    return Ok(());
}
