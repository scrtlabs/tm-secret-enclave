//go:build !sgx

package api

type EnclaveRandom struct {
	Random []byte `json:"random"`
	Proof  []byte `json:"proof"`
}

func GetRandom(blockHash []byte, height uint64) (*EnclaveRandom, error) {
	return nil, nil
}

func SubmitNextValidatorSet(valSet []byte) error {
	return nil
}

func ValidateRandom(encryptedRandom EnclaveRandom, blockHash []byte, height uint64) bool {
	return true
}
