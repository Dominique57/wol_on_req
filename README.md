# Wol On Req

WolOnReq is a projet aimed to wake up a computer on demand. It is a dedicated service that will receive requests from an
external service (such as reverse proxies) and send WOL packets.

This project is inspired by [Sablier](https://github.com/sablierapp/sablier) but dedicated at starting a computer
instead of container services.

## Usage

1. WolOnReq must run be accessible on the reverse proxy :
    - configuration file mapping IP addresses to MAC addresses as target for WOL packets,
2. The reverse proxy :
    - middleware or equivalent (see [plugins](./plugins) for supported reverse proxies),

## Installation

> TODO

## Examples

> TODO

## WIP and bugs

> TODO

## Contributing

Please open an issue first to discuss what you would like to change.

## License

> TBD once we have an MVP and a set of dependencies to consider

## Authors

- Dominique MICHEL [MAIL](mailto:dominique.michel@epita.fr)
