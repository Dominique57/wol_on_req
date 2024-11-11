# Traefik WolOnReq Plugin

- [Traefik WolOnReq Plugin](#traefik-WolOnReq-plugin)
    - [Installation](#installation)
    - [Traefik with Docker classic](#traefik-with-docker-classic)
    - [Plugin](#plugin)
    - [Development](#development)

## Installation

1. Add this snippet in the Traefik Static configuration :

> TODO

2. Configure the plugin :

> TODO: complete

```yaml
# dynamic-config.yml
http:
  middlewares:
    my-wolonreq:
      plugin:
        wolonreq:
```

## Traefik with Docker classic

> TODO

## Plugin

> TODO: once plugin is working and tested, request plugin to be added to the traefik catalog https://plugins.traefik.io/

## Development

Load the plugin in local mode ([ref](https://github.com/traefik/plugindemo?tab=readme-ov-file#local-mode)) :

```yaml
version: "3.8"

services:
  traefik:
    image: traefik:latest
    command:
      - --entryPoints.http.address=:80
      - --providers.docker=true
      - --experimental.localPlugins.wolonreq.moduleName=github.com/dominique57/wolonreqDEV
    ports:
      - "8080:80"
    volumes:
      - '/var/run/docker.sock:/var/run/docker.sock'
      - './dynamic-config.yml:/etc/traefik/dynamic-config.yml'
      - '../../..:/plugins-local/src/github.com/dominique57/wolonreqDEV'
```
