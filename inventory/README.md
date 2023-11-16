# README

Inventory state worker



## Usage

- get

```sh
golem-cli worker invoke-and-await -w inventory-1 -t inventory -p stan -f golem:template/api/get -j '["123"]'
```

- increase

```sh
golem-cli worker invoke-and-await -w inventory-1 -t inventory -p stan -f golem:template/api/increase -j '["123", 42]'
```

- decrease

```sh
golem-cli worker invoke-and-await -w inventory-1 -t inventory -p stan -f golem:template/api/increase -j '["123", 21]'
```
