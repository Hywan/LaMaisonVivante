# VZug Reader

This program aims at reading information from a [V-Zug Adora SL-WP
(41134 000054)](https://www.vzug.com/ch/en/) (the link points to the
V-Zug homepage, since the product has been removed from the
catalog). This device is our dishwasher, which is one of the most
ecological and economical we have found on the market at that
time. Bonus, it is built in Switzerland.

Data are read through the V-ZUG-Home HTTP API, which runs on the
appliance itself. Thus, the mobile app isn't required.

## Installation

The program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```shell
$ cargo build --release
```

The executable binary is located in `./target/release/vzug-reader`.

## Usage

Use `-h`/`--help` to get help:

```shell
vzug-reader 0.1.0

USAGE:
    vzug-reader [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -c, --print-config-path    Print the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -a, --address <address>    HTTP address of the VZug adora diswhwasher, e.g. `192.168.1.142:80`. This option
                               overwrites the value read from the configuration file
    -f, --format <format>      Define the kind of outputs [default: Text]  [possible values: Text, Json]
```

Use the `--address` option to specify the address. That's the only
thing you need to know!

A configuration file can be used to read the value of the `--address`
option. Use `--print-config-path` to get the path to the configuration
file.

### Format

The `vzug-reader` tool is designed to work in multiple environments.

#### Text

By default, the text format is selected.

```text
State {
    device: Device {
        model: "GS-ASLWP",
        description: "Adora SL-WP",
        serial_number: "41134 000054",
        article_number: "4113400055",
        api_version: "1.6.0",
    },
    average_consumption: Consumption {
        power: Kwh(
            0.7,
        ),
        water: Liter(
            13.0,
        ),
    },
    total_consumption: Consumption {
        power: Kwh(
            42.0,
        ),
        water: Liter(
            930.0,
        ),
    },
    current_program: Active {
        status: "active",
        id: 50,
        name: "Pogramme Eco",
        current_step: 2,
        steps: [
            79,
            82,
            79,
            78,
            74,
            75,
            72,
        ],
        eco: Option {
            set: "none",
        },
        steam_finish: Option {
            set: false,
        },
        partial_load: Option {
            set: true,
        },
    },
}
```

#### [JSON](https://www.json.org/json-en.html)

JSON can be used in a Web environment. Example with `vzug-reader
--address <addr< --format json` (formatted with `… | python -m
json.tool`):

```json
{
    "device": {
        "model": "GS-ASLWP",
        "description": "Adora SL-WP",
        "serial_number": "41134 000054",
        "article_number": "4113400055",
        "api_version": "1.6.0"
    },
    "average_consumption": {
        "power": 0.7,
        "water": 13.0
    },
    "total_consumption": {
        "power": 42.0,
        "water": 930.0
    },
    "current_program": {
        "status": "active",
        "id": 50,
        "name": "Pogramme Eco",
        "current_step": 2,
        "steps": [
            79,
            82,
            79,
            78,
            74,
            75,
            72
        ],
        "eco": {
            "set": "none"
        },
        "steam_finish": {
            "set": false
        },
        "partial_load": {
            "set": true
        }
    }
}
```

