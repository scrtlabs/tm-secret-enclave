use crate::enclave::enclave_api::{ecall_generate_random, ecall_submit_validator_set, ecall_validate_random};
use crate::Error;
use sgx_types::{sgx_status_t, SgxResult, sgx_enclave_id_t};


use libc::{dlsym, RTLD_DEFAULT, c_void};
use std::ffi::CString;
use std::ptr::{null, null_mut};

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

//type Pfn_random_number = unsafe extern "C" fn(block_hash: &[u8], height: u64) -> Result<Vec<u8>, crate::Error>;
//static mut S_PFN_RANDOM_NUMBER: Option<Pfn_random_number> = None;

type Symbol = *mut c_void;

fn ensure_symbol_found(name: &str, p_symbol: &mut Symbol) -> bool {

    if (*p_symbol).is_null() {

        let symbol_name = CString::new(name).unwrap();
        *p_symbol = unsafe { dlsym(RTLD_DEFAULT, symbol_name.as_ptr()) };

        if (*p_symbol).is_null() {
            println!("### symbol {} not found", name);
            return false;
        }
        println!("### symbol {} loaded", name);
    }
    true
}

static mut S_PFN_RANDOM_NUMBER: Symbol = null_mut();


pub fn random_number(block_hash: &[u8], height: u64) -> Result<Vec<u8>, crate::Error> {

    unsafe {

        if !ensure_symbol_found("secret_impl_random_number", &mut S_PFN_RANDOM_NUMBER) {
            return Err(Error::RandomGeneration { msg: "status unexpected".to_string() });
        }
        

        // Cast the raw pointer to the correct function type
        type Pfn = unsafe extern "C" fn(block_hash: &[u8], height: u64) -> Result<Vec<u8>, crate::Error>;
        let function: Pfn = std::mem::transmute(S_PFN_RANDOM_NUMBER);

        function(block_hash, height)

    }
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
