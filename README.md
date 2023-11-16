# README

A modern and modular ERP



## Features

- built on Wasm workers running in Golem Cloud



## Usage

- install dependencies

```sh
rustup target add wasm32-wasi
brew install protobuf
cargo install cargo-component
```

- build

```sh
cargo component build --release
```

- create Golem template

```sh
golem-cli template add \
  --template-name stan \
  target/wasm32-wasi/release/stan.wasm
# or if already exists
golem-cli template update \
  --template-name stan \
  target/wasm32-wasi/release/stan.wasm
```

- create Golem worker

```sh
golem-cli worker add \
  --worker-name stan-1 \
  --template-name stan
# or if already exists
golem-cli worker delete \
  --worker-name stan-1 \
  --template-name stan
golem-cli worker add \
  --worker-name stan-1 \
  --template-name stan
```
