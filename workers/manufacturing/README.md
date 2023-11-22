# README

Manufacturing workflow worker



## Usage

- create

```sh
golem-cli worker invoke-and-await -w manufacturing-1 -t manufacturing -p stan -f stan:manufacturing/api/create -j '["123", 42]'
```
