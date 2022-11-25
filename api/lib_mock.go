//go:build !sgx

package api

func GetRandom() (int64, error) {
	return 0, nil
}
