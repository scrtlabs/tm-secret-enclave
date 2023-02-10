package main

import (
	"fmt"
	enclave "github.com/scrtlabs/tm-secret-enclave"
)

// This is just a demo to ensure we can compile a static go binary
func main() {
	fmt.Println("Attempting to get check health of enclave")
	err := enclave.HealthCheck()
	if err != nil {
		return
	}

	fmt.Println("Attempting to get data from the enclave")
	random, _, err := enclave.GetRandom([]byte{0xaa, 0xbb, 0xcc}, 1)
	if err != nil {
		return
	}

	fmt.Println("finished with random result:", random)
}
