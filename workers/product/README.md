# README

Product state worker



## Usage

- add

```sh
golem-cli worker invoke-and-await -w product-1 -t product -p stan -f stan:product/api/add -j '[{"id": "123", "name": "foo"}]'
```

- get

```sh
golem-cli worker invoke-and-await -w product-1 -t product -p stan -f stan:product/api/get -j '["123"]'
```
