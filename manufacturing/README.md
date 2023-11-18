# README

Manufacturing state and workflow worker



## Usage

### Material

- add

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f golem:template/material/add -j '[{"name": "foo"}]'
```

- get

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f golem:template/material/get -j '["123"]'
```

### Routing

- add

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f golem:template/routing/add -j '[{"material-id": "123", "parts": [{"material-id": "456", amount: 21}, {"material-id": "789", amount: 42}]}]'
```

- get

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f golem:template/routing/get -j '["123"]'
```

### Build

- create

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f golem:template/build/create -j '["123", 42]'
```
