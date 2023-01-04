//go:build !sgx

package api

//type EnclaveRandom struct {
//	Random []byte `json:"random"`
//	Proof  []byte `json:"proof"`
//}

func GetRandom() ([]byte, error) {
	return nil, nil
}

func SubmitNextValidatorSet(valSet []byte) error {
	return nil
}

func ValidateRandom(enclaveRandom []byte) bool {
	return true
}
