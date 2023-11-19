# README

Routing state worker



## Usage

- add

```sh
golem-cli worker invoke-and-await -w routing-1 -t routing -p stan -f golem:routing/api/add -j '[{"material-id": "123", "parts": [{"material-id": "456", amount: 21}, {"material-id": "789", amount: 42}]}]'
```

- get

```sh
golem-cli worker invoke-and-await -w routing-1 -t routing -p stan -f golem:routing/api/get -j '["123"]'
```
