// Trick to get the IDE to use sgx_tstd even when it doesn't know we're targeting SGX
#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

extern crate sgx_types;
//
// use ctor::*;
// use enclave_utils::logger::get_log_level;

use sgx_types::{sgx_status_t};

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
