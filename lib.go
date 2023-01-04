package tm_secret_enclave

import "github.com/scrtlabs/tm-secret-enclave/api"

type EnclaveRandom = []byte

func GetRandom() ([]byte, error) {
	return api.GetRandom()
}

func SubmitNextValidatorSet(valSet []byte) error {
	return api.SubmitNextValidatorSet(valSet)
}

func ValidateRandom(enclaveRandom []byte) bool {
	return api.ValidateRandom(enclaveRandom)
}
