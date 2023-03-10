network ?= devnet  # network := devnet|mainnet|testnet
contract_addr_filepath ?= $(release_dirpath)/contract_addr.txt
wasm_filename ?= cw_gelotto_player.wasm
release_dirpath ?= ./release
sender ?= juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y

# build optimized WASM artifact
build:
	./bin/build

# deploy WASM file (generated from `make build`)
deploy:
	./bin/deploy ./artifacts/$(wasm_filename) $(network) $(sender)

# instantiate last contract to be deployed using code ID in release dir code-id file
instantiate:
	./bin/instantiate $(network) $(sender)

# run all unit tests
test:
	RUST_BACKTRACE=1 cargo unit-test

# Generate the contract's JSONSchema JSON files in schemas/
schemas:
	cargo schema

# Run/start local "devnet" validator docker image	
validator:
	./bin/validator

authorize:
	./client.sh authorize $(network) $(contract_addr_filepath) $(sender) $(principal) $(action)

is-authorized:
	./client.sh is-authorized $(network) $(contract_addr_filepath) $(sender) $(principal) $(action)