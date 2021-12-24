# Nun-db Prometheus Exporter

This is a simple server that scrapes Nun-db stats and exports them via HTTP for Prometheus consumption.

## Env vars
The exporter needs 3 evenvars

`NUN_USER` -> NunDb user name
`NUN_PWD` ->  NunDb password
`NUN_URL` -> NunDb url to be read

## It as a Docker run

```
docker run --env NUN_USER=$user --env NUN_PWD=$pwd --env NUN_URL="$nun_url" -p 9898:9898 mateusfreira/nun-db-prometheus-exporter:latest
``` 

