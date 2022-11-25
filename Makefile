BUILD_PROFILE ?= release

TOP_DIR := ../
include $(TOP_DIR)/buildenv.mk

FEATURES ?=
FEATURES_U += $(FEATURES)
#FEATURES_U += backtraces
FEATURES_U := $(strip $(FEATURES_U))

DOCKER_TAG := 0.8.2
USER_ID := $(shell id -u)
USER_GROUP = $(shell id -g)

DLL_EXT = ""
ifeq ($(OS),Windows_NT)
	DLL_EXT = dll
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		DLL_EXT = so
	endif
	ifeq ($(UNAME_S),Darwin)
		DLL_EXT = dylib
	endif
endif

SGX_SDK ?= $(HOME)/.sgxsdk/sgxsdk

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_CFLAGS := -m32
else
	SGX_COMMON_CFLAGS := -m64
endif

ifeq ($(SGX_DEBUG), 1)
	SGX_COMMON_CFLAGS += -O0 -g
else
	SGX_COMMON_CFLAGS += -O2
endif

SGX_COMMON_CFLAGS += -fstack-protector

CUSTOM_EDL_PATH := ../../deps/incubator-teaclave-sgx-sdk/sgx_edl/edl

App_Include_Paths := -I./ -I./include -I$(SGX_SDK)/include -I$(CUSTOM_EDL_PATH)
App_C_Flags := $(SGX_COMMON_CFLAGS) -fPIC -Wno-attributes $(App_Include_Paths)

Enclave_Path := ../enclave
Enclave_EDL_Products := lib/enclave/Enclave_u.c lib/enclave/Enclave_u.h

.PHONY: all
all: build cmd

.PHONY: build
build: build-rust build-go

.PHONY: build-rust
build-rust: build-enclave
	cargo build -Z unstable-options --profile $(BUILD_PROFILE) --features "$(FEATURES_U)"
	cp target/$(BUILD_PROFILE)/librandom_api.so api
	@ #this pulls out ELF symbols, 80% size reduction!

.PHONY: build-enclave
build-enclave: librust_cosmwasm_enclave.signed.so lib

librust_cosmwasm_enclave.signed.so: inner-build-enclave
	cp $(Enclave_Path)/$@ ./

.PHONY: inner-build-enclave
inner-build-enclave:
	FEATURES="$(FEATURES)" $(MAKE) -C $(Enclave_Path) enclave

# This file will be picked up by the crate's build script and linked into the library.
lib/libEnclave_u.a: $(Enclave_EDL_Products)
	$(CC) $(App_C_Flags) -c lib/enclave/Enclave_u.c -o lib/enclave/Enclave_u.o
	$(AR) rcsD $@ lib/enclave/Enclave_u.o

# We make sure that the enclave is built before we compile the edl,
# because the EDL depends on a header file that is generated in that process.
$(Enclave_EDL_Products): $(Enclave_Path)/Enclave.edl
	mkdir -p "lib/enclave"
	sgx_edger8r --untrusted $< --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --untrusted-dir ./lib/enclave

# implement stripping based on os
.PHONY: strip
ifeq ($(DLL_EXT),so)
strip:
	strip api/libgo_cosmwasm.so
else
# TODO: add for windows and osx
strip:
endif

.PHONY: build-go
build-go:
	go build -tags 'sgx' ./...

.PHONY: cmd
cmd:
	RUST_BACKTRACE=1 go build -o main -buildmode=pie -ldflags "-linkmode=external -extldflags '-static-pie'" ./cmd

.PHONY: docker-image-alpine
docker-image-alpine:
	docker build . -t cosmwasm/go-ext-builder:test-alpine

# and use them to compile release builds
.PHONY: release
release:
	rm -rf target/release
	docker run --rm -u $(USER_ID):$(USER_GROUP) -v /opt/intel/sgxsdk:/opt/intel/sgxsdk -v $(shell pwd):/code/code/ -v $(shell pwd)/../../deps:/deps cosmwasm/go-ext-builder:test-alpine

.PHONY: test-safety
test-safety:
	GODEBUG=cgocheck=2 go test -race -v -count 1 ./api

.PHONY: clean
clean:
	rm -rf lib $(Enclave_EDL_Products) $(Query_Enclave_EDL_Products) *.o *.so *.h
	cargo clean

.PHONY: clean-all
clean-all: clean
	$(MAKE) -C $(Enclave_Path) clean
	$(MAKE) -C $(Query_Enclave_Path) clean
