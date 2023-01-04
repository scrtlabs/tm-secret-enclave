use enclave_crypto::AESKey;
use enclave_crypto::consts::{DEFAULT_SGX_SECRET_PATH, SCRT_SGX_STORAGE_ENV_VAR};
use lazy_static::lazy_static;
use std::{env, path};

const REK_SEALED_FILE_NAME: &str = "rek.sealed";
const IRS_SEALED_FILE_NAME: &str = "irs.sealed";


fn path_from_env(file_name: &str) -> String {
    path::Path::new(
        &env::var(SCRT_SGX_STORAGE_ENV_VAR).unwrap_or_else(|_| DEFAULT_SGX_SECRET_PATH.to_string())
    )
        .join(file_name)
        .to_str()
        .unwrap_or(DEFAULT_SGX_SECRET_PATH)
        .to_string()
}

lazy_static! {
    static ref REK_SEALING_PATH: String = path_from_env(REK_SEALED_FILE_NAME);
    static ref IRS_SEALING_PATH: String = path_from_env(IRS_SEALED_FILE_NAME);
    /// This variable indicates if the enclave configuration has already been set
    pub static ref REK: AESKey = AESKey::unseal(REK_SEALING_PATH).unwrap();
    pub static ref IRS: AESKey = AESKey::unseal(IRS_SEALING_PATH).unwrap();
}
