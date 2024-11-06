use crate::Error;
use sgx_types::{sgx_status_t, SgxResult, sgx_enclave_id_t};


use libc::{dlsym, RTLD_DEFAULT, c_void};
use std::ffi::CString;
use std::ptr::{null_mut};

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
        type Pfn = unsafe extern "C" fn(&[u8], height: u64) -> Result<Vec<u8>, sgx_status_t>;
        let function: Pfn = std::mem::transmute(S_PFN_RANDOM_NUMBER);
        
        function(block_hash, height).map_err(|_| Error::RandomGeneration { msg: "status unexpected".to_string() })
    }
}

static mut S_PFN_NEXT_VALIDATOR_SET: Symbol = null_mut();

pub fn next_validator_set(val_set: &[u8], height: u64) -> SgxResult<()> {

    unsafe {

        if !ensure_symbol_found("secret_impl_next_validator_set", &mut S_PFN_NEXT_VALIDATOR_SET) {
            return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
        }
        

        // Cast the raw pointer to the correct function type
        type Pfn = unsafe extern "C" fn(&[u8], height: u64) -> Result<(), sgx_status_t>;
        let function: Pfn = std::mem::transmute(S_PFN_NEXT_VALIDATOR_SET);
        
        function(val_set, height)
    }
}

static mut S_PFN_VALIDATE_RANDOM: Symbol = null_mut();

pub fn enclave_validate_random(random: &[u8], proof: &[u8], block_hash: &[u8], height: u64) -> SgxResult<()> {

    unsafe {

        if !ensure_symbol_found("secret_impl_validate_random", &mut S_PFN_VALIDATE_RANDOM) {
            return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
        }
        

        // Cast the raw pointer to the correct function type
        type Pfn = unsafe extern "C" fn(&[u8], &[u8], &[u8], height: u64) -> Result<(), sgx_status_t>;
        let function: Pfn = std::mem::transmute(S_PFN_VALIDATE_RANDOM);
        
        function(random, proof, block_hash, height)
    }
}
