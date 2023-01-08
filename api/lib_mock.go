//go:build !sgx

package api

type EnclaveRandom struct {
	Random []byte `json:"random"`
	Proof  []byte `json:"proof"`
}

func GetRandom(blockHash []byte, height uint64) ([]byte, []byte, error) {
	return nil, nil, nil
}

func SubmitValidatorSet(valSet []byte) error {
	return nil
}

func ValidateRandom(random []byte, proof []byte, blockHash []byte, height uint64) bool {
	return true
}
