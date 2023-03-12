use sgx_types::{
    sgx_attributes_t, sgx_launch_token_t, sgx_misc_attribute_t, sgx_status_t, SgxResult,
};
use sgx_urts::SgxEnclave;
use std::path::Path;

use log::{trace, warn};
use std::env;

#[cfg(feature = "production")]
const ENCLAVE_DEBUG: i32 = 0;
#[cfg(not(feature = "production"))]
const ENCLAVE_DEBUG: i32 = 1;

pub fn init_enclave(enclave_file: &str) -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug: i32 = ENCLAVE_DEBUG;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };

    let enclave_directory = env::var("SCRT_ENCLAVE_DIR").unwrap_or_else(|_| '.'.to_string());

    let mut enclave_file_path = None;
    let dirs = [
        enclave_directory.as_str(),
        "/lib",
        "/usr/lib",
        "/usr/local/lib",
    ];
    for dir in dirs.iter() {
        let candidate = Path::new(dir).join(enclave_file);
        trace!("Looking for the enclave file in {:?}", candidate.to_str());
        if candidate.exists() {
            enclave_file_path = Some(candidate);
            break;
        }
    }

    let enclave_file_path = enclave_file_path.ok_or_else(|| {
        warn!(
            "Cannot find the enclave file. Try pointing the SCRT_ENCLAVE_DIR environment variable to the directory that has {:?}",
            enclave_file
        );
        sgx_status_t::SGX_ERROR_INVALID_ENCLAVE
    })?;

    SgxEnclave::create(
        enclave_file_path,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}
