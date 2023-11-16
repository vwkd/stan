# README

Order workflow worker



## Usage

- create

```sh
golem-cli worker invoke-and-await -w order-1 -t order -p stan -f golem:template/api/create -j '[[{"product-id": "123", "price": 15, "quantity": 3}, {"product-id": "456", "price": 3.5, "quantity": 9}]]'
```
