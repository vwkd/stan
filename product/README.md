# README

Product state worker



## Usage

- query

```sh
golem-cli worker invoke-and-await \
  --template-name=product \
  --worker-name=product-1 \
  --function=golem:template/api/add \
  --parameters='[{"id": "123", "name": "foo"}]'
golem-cli worker invoke-and-await \
  --template-name=product \
  --worker-name=product-1 \
  --function=golem:template/api/get \
  --parameters='["123"]'
```
