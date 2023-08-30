//go:build sgx

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lrandom_api // -lsgx_epid -- add this back when compiling with 2.20
import "C"
