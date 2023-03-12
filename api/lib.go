//go:build sgx

package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"fmt"
	"time"
)

// nice aliases to the rust names
type i32 = C.int32_t

type (
	i64    = C.int64_t
	u64    = C.uint64_t
	u32    = C.uint32_t
	u8     = C.uint8_t
	u8_ptr = *C.uint8_t
	usize  = C.uintptr_t
	cint   = C.int
	cbool  = C.bool
)

const RETRIES = 5
const SLEEP_MS = 200

type EnclaveRandom struct {
	Random []byte `json:"random"`
	Proof  []byte `json:"proof"`
}

func GetHealthCheck() (int64, error) {
	errmsg := C.Buffer{}

	_, err := C.get_health_check(&errmsg)
	if err != nil {
		return 0, err
	}

	return 0, nil
}

func ValidateRandom(encryptedRandom EnclaveRandom, blockHash []byte, height uint64) bool {
	// errmsg := C.Buffer{}
	randomSlice := sendSlice(encryptedRandom.Random)
	defer freeAfterSend(randomSlice)
	proofSlice := sendSlice(encryptedRandom.Proof)
	defer freeAfterSend(proofSlice)
	blockHashSlice := sendSlice(blockHash)
	defer freeAfterSend(blockHashSlice)

	// need to wrap with C.uint64_t otherwise compiler mixes up mapping of types between languages
	res := C.validate_random(randomSlice, proofSlice, blockHashSlice, u64(height))
	return bool(res)
	//if err != nil {
	//	//todo: log or return error
	//	return false
	//}
	//
	//return true
}

func GetRandom(blockHash []byte, height uint64) (*EnclaveRandom, error) {
	errmsg := C.Buffer{}
	blockHashSlice := sendSlice(blockHash)
	defer freeAfterSend(blockHashSlice)

	res, err := C.get_random_number(blockHashSlice, u64(height), &errmsg)
	if err != nil {
		return nil, fmt.Errorf("error")
	}

	vec := receiveVector(res)
	//data := binary.BigEndian.Uint64(vec)
	//fmt.Println("Got data from enclave:", data, "\n")
	if len(vec) != 80 {
		return nil, fmt.Errorf("Got random from enclave with a weird length: ", len(vec))
	}

	ret := &EnclaveRandom{
		Random: vec[0:48],
		Proof:  vec[48:80],
	}

	return ret, nil
}

func SubmitValidatorSet(valSet []byte, height uint64) error {
	errmsg := C.Buffer{}
	valSetSlice := sendSlice(valSet)
	defer freeAfterSend(valSetSlice)

	for i := 0; i <= RETRIES; i++ {
		C.submit_next_validator_set(valSetSlice, u64(height), &errmsg)
		if errmsg.len == 0 {
			// No error occurred, return nil
			return nil
		}

		time.Sleep(SLEEP_MS * time.Millisecond)
		// Error occurred, retry in the next iteration
	}

	// If we reach here, all retries have failed, return error
	return fmt.Errorf("failed submitting validator set to enclave after %d retries", retries)
}
