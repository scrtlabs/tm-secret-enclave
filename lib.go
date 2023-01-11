package tm_secret_enclave

import "github.com/scrtlabs/tm-secret-enclave/api"

type EnclaveRandom = api.EnclaveRandom

func GetRandom(blockHash []byte, height uint64) ([]byte, []byte, error) {
	res, err := api.GetRandom(blockHash, height)
	return res.Random, res.Proof, err
}

func SubmitValidatorSet(valSet []byte, height uint64) error {
	return api.SubmitValidatorSet(valSet, height)
}

func ValidateRandom(random []byte, proof []byte, blockHash []byte, height uint64) bool {
	return api.ValidateRandom(EnclaveRandom{
		Proof:  proof,
		Random: random,
	}, blockHash, height)
}
