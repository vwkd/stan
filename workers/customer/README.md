# README

Customer state worker



## Usage

- add

```sh
golem-cli worker invoke-and-await -w customer-1 -t customer -p stan -f stan:customer/api/add -j '[{"name": "foo"}]'
```

- get

```sh
golem-cli worker invoke-and-await -w customer-1 -t customer -p stan -f stan:customer/api/get -j '[123]'
```
