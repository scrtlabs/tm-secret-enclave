//go:build !sgx

package api

import "C"
import (
	"fmt"
	"math/rand"
)

type EnclaveRandom struct {
	Random []byte `json:"random"`
	Proof  []byte `json:"proof"`
}

func GetHealthCheck() (int64, error) {
	return 0, nil
}

func GetRandom(blockHash []byte, height uint64) (*EnclaveRandom, error) {
	buf := make([]byte, 32)
	_, err := rand.Read(buf)
	if err != nil {
		return nil, fmt.Errorf("failed to generate random")
	}

	return &EnclaveRandom{
		Random: buf,
		Proof:  nil,
	}, nil
}

func SubmitValidatorSet(valSet []byte, height uint64) error {
	return nil
}

func ValidateRandom(rand EnclaveRandom, blockHash []byte, height uint64) bool {
	return true
}
