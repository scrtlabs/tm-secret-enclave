//go:build !sgx

package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"encoding/binary"
	"fmt"
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
)

//func GetHealthCheck() (int64, error) {
//	errmsg := C.Buffer{}
//
//	res, err := C.get_health_check(&errmsg)
//	if err != nil {
//		return 0, errorWithMessage(err, errmsg)
//	}
//
//	vec := receiveVector(res)
//	data := binary.BigEndian.Uint64(vec)
//	fmt.Println(data)
//
//	return int64(data), nil
//}

func GetRandom() (uint64, error) {
	errmsg := C.Buffer{}

	res, err := C.get_random_number(&errmsg)
	if err != nil {
		return 0, fmt.Errorf("error")
	}

	vec := receiveVector(res)
	data := binary.BigEndian.Uint64(vec)
	fmt.Println("Got data from enclave:", data, "\n")

	return data, nil
}
