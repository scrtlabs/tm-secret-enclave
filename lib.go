package tm_secret_enclave

import "github.com/scrtlabs/tm-secret-enclave/api"

func GetRandom() (int64, error) {
	return api.GetRandom()
}
