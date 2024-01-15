//go:build sgx

package api

//// -lsgx_epid -- add this back when compiling with 2.20

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lrandom_api 
import "C"
