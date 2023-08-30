//go:build sgx

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lrandom_api -lsgx_epid
import "C"
