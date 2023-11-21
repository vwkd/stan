# README

Material state worker



## Usage

- add

```sh
golem-cli worker invoke-and-await -w material-1 -t material -p stan -f golem:material/api/add -j '[{"name": "foo"}]'
```

- get

```sh
golem-cli worker invoke-and-await -w material-1 -t material -p stan -f golem:material/api/get -j '[123]'
```
