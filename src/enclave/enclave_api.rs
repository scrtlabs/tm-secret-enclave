use sgx_types::{sgx_enclave_id_t, sgx_status_t};

// ecalls

extern "C" {
    pub fn ecall_health_check(eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;

    pub fn ecall_generate_random(eid: sgx_enclave_id_t, retval: *mut u64) -> u64;

    pub fn ecall_submit_validator_set(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        val_set: *const u8,
        val_set_len: u32,
    ) -> sgx_status_t;

    pub fn ecall_validate_encrypted_random(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        encrypted_random: *const u8,
        encrypted_random_len: u32,
    ) -> sgx_status_t;
}
