# Alfen

This program aims at reading information from an Alfen charging
station (NG9xx series). In our home, this Alfen Eve Single S-line aims
at charging our electric vehicle.

Data are read through Modbus TCP. An IP address is then required to
reach the Alfen charging station. The port 502 must be
opened. Configuration is explained in the documentation (see [the
`doc` directory](./doc)).

## Installation

The program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```sh
$ cargo build --release
```

The executable binary is located in `/target/release/alfen`.

## Usage

Use `-h`/`--help` to get help:

```
alfen 0.0.1
This command allows to read values, or write new values to an Alfen NG9xx charging station

USAGE:
    alfen [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help                 Prints help information
    -c, --print-config-path    Print the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -a, --address <address>    Modbus address of the Alfen, e.g. `192.168.1.142:502`. This option overwrites the value
                               read from the configuration file

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    read     Read values from the Alfen
    write    Write values to the Alfen
```

Use the `read` or `write` subcommands to do something useful. Use the
`--address` option to specify the address.

A configuration file can be used to read the value of the `--address`
option. Use `--print-config-path` to get the path to the configuration
file.

Let's see `alfen read --help`:

```
alfen-read 0.0.1
Read values from the Alfen

USAGE:
    alfen read [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -t, --into-thing    Turns this program into a Thing, i.e. a new Web of Things device
    -V, --version       Prints version information

OPTIONS:
    -f, --format <format>            Define the kind of outputs [default: Text]  [possible values: Text, Json]
    -p, --thing-port <thing-port>    Port of the Thing. Requires `--into-thing` to be effective. This option overwrites
                                     the value read from the configuration file
```

And let's see `alfen write --help`:

```
alfen-write 0.0.1
Write values to the Alfen

USAGE:
    alfen write [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --socket-current <socket-current>    Update the applied current of the socket
```

### Format

The `alfen read` tool is designed to work in multiple environments.

#### Text

By default, the text format is selected.

```
State {
    station_information: StationInformation {
        name: "ACE0196818",
        manufacturer: "Alfen NV",
        platform_type: "NG910",
        serial_number: "ACE0196818",
        firmware_version: "5.8.1-4123",
        date: 2022-10-10T11:37:36+00:01,
        uptime: 2097199.489s,
    },
    station_status: StationStatus {
        max_current: 15A,
        temperature: 36.75°C,
        is_ocpp_connected: false,
        number_of_sockets: 1,
    },
    socket: Socket {
        availability: Operative,
        status: Disconnected,
        number_of_phases: Three,
        l1: SocketPhase {
            voltage: 229.90999V,
            current: 0A,
        },
        l2: SocketPhase {
            voltage: 228.94V,
            current: 0A,
        },
        l3: SocketPhase {
            voltage: 229.19V,
            current: 0A,
        },
        power: 0W,
        frequency: 52.640003Hz,
        total_delivered_energy: 301606Wh,
        session: SocketSession {
            max_current: 6A,
            actual_applied_max_current: 10A,
            remaining_time_before_fallback_to_safe_current: 0,
        },
    },
}
```

#### [JSON](https://www.json.org/json-en.html)

JSON can be used in a Web environment. Example with `alfen read
--address <addr> --format json` (formatted with `… | python -m
json.tool`):

```json
{
    "station_information": {
        "name": "ACE0196818",
        "manufacturer": "Alfen NV",
        "platform_type": "NG910",
        "serial_number": "ACE0196818",
        "firmware_version": "5.8.1-4123",
        "date": "2022-10-10T11:40:07+00:01",
        "uptime": {
            "secs": 2130759,
            "nanos": 118000000
        }
    },
    "station_status": {
        "max_current": 15.0,
        "temperature": 37.0,
        "is_ocpp_connected": false,
        "number_of_sockets": 1
    },
    "socket": {
        "availability": "Operative",
        "status": "Disconnected",
        "number_of_phases": "Three",
        "l1": {
            "voltage": 230.12999,
            "current": 0.0
        },
        "l2": {
            "voltage": 229.01,
            "current": 0.0
        },
        "l3": {
            "voltage": 229.37,
            "current": 0.0
        },
        "power": 0.0,
        "frequency": 52.640003,
        "total_delivered_energy": 301606.0,
        "session": {
            "max_current": 6.0,
            "actual_applied_max_current": 10.0,
            "remaining_time_before_fallback_to_safe_current": 0
        }
    }
}
```

### [Web of Things](https://www.w3.org/WoT/)

To turn the Alfen device into a standardized connected things, use the
`--into-thing` option: It will start a local Things server. The
`--thing-port` is useful to set the server's port.

One the Things server is running, use a gateway like the [WebThings
Gateway](https://iot.mozilla.org/gateway/) to interact with the Alfen
device. Enjoy!

```sh
$ /target/release/alfen read --address 192.168.1.107:502 --into-thing --thing-port 8088
Starting the Things server (port 8088)…
```
