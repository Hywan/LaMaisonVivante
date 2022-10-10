# Kia

This program aims at reading information from a Kia electric
vehicle. We have a [Kia EV6](https://www.kia.ch/fr/notre-gamme/ev6/),
thus we are using [the Kia Connect
API](https://www.kia.ch/fr/experience/kia-connect/) to read data from
the vehicle. This API is different based on your region. Currently, we
only support the Europe, which requires the most… “astonishing” API.

## Installation

The program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```sh
$ cargo build --release
```

The executable binary is located in `/target/release/kia`.

## Usage

Use `-h`/`--help` to get help:

```
kia 0.1.0

USAGE:
    kia [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -t, --into-thing           Turns this program into a Thing, i.e. a new Web of Things device
    -c, --print-config-path    Print the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -s, --password <password>        Password of the Kia Connect account
    -p, --thing-port <thing-port>    Port of the Thing. Requires `--into-thing` to be effective. This option overwrites
                                     the value read from the configuration file
    -u, --username <username>        Username of the Kia Connect account
```

Use the `--username` and `--password` from a Kia Connect account to
fetch the data.

A configuration file can be used to read those options. Use
`--print-config-path` to get the path to the configuration file.

By default, it will output something like this:

```
Opening the garage…
Looking for vehicles…
Found 1 vehicle(s).

## EV6 (…)

Vehicle {
    vin: "…",
    vehicle_id: "…",
    vehicle_name: "EV6",
    nickame: "EV6",
    master: true,
    car_share: 0,
    ..
}

with state:

State {
    status: Status {
        battery: Battery {
            is_charging: false,
            state_of_charge: 95%,
            remaining_range: 603,
            estimated_charging_duration: 9300s,
        },
        doors: Doors {
            is_front_left_opened: false,
            is_front_right_opened: false,
            is_back_left_opened: false,
            is_back_right_opened: false,
        },
        windows: Windows {
            is_front_left_opened: false,
            is_front_right_opened: false,
            is_back_left_opened: false,
            is_back_right_opened: false,
        },
        targeted_temperature: 15°C,
        is_air_conditionning_enabled: false,
        is_engine_running: false,
        is_locked: true,
        is_trunk_opened: false,
        is_frunk_opened: false,
        is_defrost_enabled: false,
        is_steer_wheel_heat_enabled: false,
        is_side_back_window_heat_enabled: false,
        is_hazard_detected: false,
        has_smart_key_battery_issue: false,
        has_washer_fluid_issue: false,
        has_break_oil_issue: false,
        has_tail_lamp_issue: false,
    },
    location: Location {
        coordinates: Coordinates {
            latitude: 46.780674°,
            longitude: 6.643161°,
            altitude: Some(
                0m,
            ),
        },
        precision_dilution: Some(
            PrecisionDilution {
                horizontal: 10,
                position: 10,
            },
        ),
    },
    odometer: 15614.4km,
}
```

### [Web of Things](https://www.w3.org/WoT/)

To turn the Kia vehicle into a standardized connected things, use the
`--into-thing` option: It will start a local Things server. The
`--thing-port` is useful to set the server's port.

One the Things server is running, use a gateway like the [WebThings
Gateway](https://iot.mozilla.org/gateway/) to interact with the Alfen
device. Enjoy!

```sh
$ /target/release/kia --username '<username>' --password '<password>' --into-thing --thing-port 8089
Starting the Things server (port 8089)…
```
