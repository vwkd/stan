# README

A modern, modular and extensible ERP



## Features

- built as Wasm components
- running durably in Golem Cloud



## Usage

- install dependencies

```sh
rustup target add wasm32-wasi
brew install protobuf
cargo install cargo-component
```

- build

```sh
cargo component build --workspace --release
```

- test

```sh
cargo test --workspace -- --test-threads=1
```

- create Golem template

```sh
golem-cli template add -t foo -p stan target/wasm32-wasi/release/foo.wasm
# or if already exists
golem-cli template update -t foo -p stan target/wasm32-wasi/release/foo.wasm
```

- create Golem worker

```sh
golem-cli worker add -w foo-1 -t foo -p stan
# or if already exists
golem-cli worker delete -w foo-1 -t foo -p stan
golem-cli worker add -w foo-1 -t foo -p stan
```
