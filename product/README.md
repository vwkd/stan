# README

Product state worker



## Usage

- query

```sh
golem-cli worker invoke-and-await \
  --template-name=product \
  --worker-name=product-1 \
  --function=golem:template/api/get \
  --parameters='[]'
```
