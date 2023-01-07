use sgx_types::{sgx_enclave_id_t, sgx_status_t};

// ecalls

extern "C" {
    pub fn ecall_health_check(eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;

    pub fn ecall_generate_random(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        block_hash: *const u8,
        block_hash_len: u32,
        height: u64,
        random: &mut [u8; 48],
        proof: &mut [u8; 32]
    ) -> sgx_status_t;

    pub fn ecall_submit_validator_set(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        val_set: *const u8,
        val_set_len: u32,
    ) -> sgx_status_t;

    pub fn ecall_validate_random(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        random: *const u8,
        random_len: u32,
        proof: *const u8,
        proof_len: u32,
        block_hash: *const u8,
        block_hash_len: u32,
        height: u64,
    ) -> sgx_status_t;
}
