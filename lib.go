package tm_secret_enclave

import "github.com/scrtlabs/tm-secret-enclave/api"

func GetRandom() (uint64, error) {
	return api.GetRandom()
}
