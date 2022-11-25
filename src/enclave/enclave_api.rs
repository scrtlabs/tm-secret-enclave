use sgx_types::{
    sgx_enclave_id_t, sgx_status_t
};

// ecalls

extern "C" {
    pub fn ecall_health_check(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
    ) -> sgx_status_t;
}
