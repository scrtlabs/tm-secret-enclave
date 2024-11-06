use crate::enclave::enclave_api::{ecall_generate_random, ecall_submit_validator_set, ecall_validate_random};
use crate::Error;
use sgx_types::{sgx_status_t, SgxResult, sgx_enclave_id_t};

static mut S_EID: Option<sgx_enclave_id_t> = None;

pub fn set_enclave(eid: u64) {
    println!("##### TM got eid={}", eid);
    unsafe {
        S_EID = Some(eid as sgx_enclave_id_t);
    }
}

fn get_enclave() -> Result<sgx_enclave_id_t, crate::Error> {
    unsafe {
        if let Some(ret_val) = S_EID {
            Ok(ret_val)
        } else {
            println!("##### TM no eid");
            Err(Error::enclave_err("sgx enclave not set"))
        }
    }
}

pub fn random_number(block_hash: &[u8], height: u64) -> Result<Vec<u8>, crate::Error> {

    println!("##### TM random_number");

    let eid = get_enclave()?;
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

    println!("##### TM random_number ret={}, status={}", retval, status);

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

    println!("##### TM next_validator_set");

    let eid = get_enclave().map_err(|_| sgx_status_t::SGX_ERROR_ECALL_NOT_ALLOWED)?;
    let mut retval = sgx_status_t::SGX_SUCCESS;


    let status = unsafe {
        ecall_submit_validator_set(eid, &mut retval, val_set.as_ptr(), val_set.len() as u32, height)
    };

    println!("##### TM next_validator_set ret={}, status={}", retval, status);

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

    println!("##### TM enclave_validate_random");

    let eid = get_enclave().map_err(|_| sgx_status_t::SGX_ERROR_ECALL_NOT_ALLOWED)?;
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

    println!("##### TM enclave_validate_random ret={}, status={}", retval, status);

    if status != sgx_status_t::SGX_SUCCESS {
        return Err(status);
    }

    if retval != sgx_status_t::SGX_SUCCESS {
        return Err(retval);
    }

    return Ok(());
}
