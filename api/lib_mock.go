//go:build !sgx

package api

func GetRandom() (uint64, error) {
	return 0, nil
}
