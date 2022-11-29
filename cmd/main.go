package main

import (
	"fmt"
	enclave "github.com/scrtlabs/tm-secret-enclave"
)

// This is just a demo to ensure we can compile a static go binary
func main() {
	random, err := enclave.GetRandom()
	if err != nil {
		return
	}

	fmt.Println("finished with random result:", random)
}
