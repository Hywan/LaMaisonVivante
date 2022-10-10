# Weather

This program aims at reading data from
[OpenWeatherMap](https://openweathermap.org/) and aggregating them
inside a single interface.

## Installation

The program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```shell
$ cargo build --release
```

The executable binary is located in `/target/release/weather`.

## Usage

Use `-h`/`--help` to get help:

```
weather 0.1.0

USAGE:
    weather [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -t, --into-thing           Turns this program into a Thing, i.e. a new Web of Things device
    -c, --print-config-path    Prints the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -k, --openweathermap-api-key <openweathermap-api-key>    The OpenWeatherMap API key
    -p, --thing-port <thing-port>
            Port of the Thing. Requires `--into-thing` to be effective. This option overwrites the value read from the
            configuration file
```

Use the `--openweathermap-api-key` option to specify the
OpenWeatherMap API key. That's all you need to know!

A configuration file can be used to read default optn values. Use
`--print-config-path` to get the path to the configuration file.

### Format

The `weather` tool is designed to either output the data, or to expose
a WebThing server.

By default, it fetches text description in French.

```
State {
    alerts: None,
    current_weather: Weather {
        clouds: 75,
        datetime: 2022-10-10T09:09:11Z,
        temperature: 13.32,
        apparent_temperature: 13.06,
        humidity: 90,
        dew_point: 11.72,
        pressure: 1020,
        sunrise: Some(
            1665380586,
        ),
        sunset: Some(
            1665420974,
        ),
        uv_index: 1.48,
        visibility: 10000,
        wind_degree: 280,
        wind_speed: 0.45,
        wind_gust: None,
        snow: None,
        rain: None,
        conditions: [
            WeatherCondition {
                description: "nuageux",
                id: BrokenClouds,
            },
        ],
    },
    hourly_weather: [
        Weather {
            clouds: 75,
            datetime: 2022-10-10T09:00:00Z,
            temperature: 13.32,
            apparent_temperature: 13.06,
            humidity: 90,
            dew_point: 11.72,
            pressure: 1020,
            sunrise: None,
            sunset: None,
            uv_index: 1.48,
            visibility: 10000,
            wind_degree: 201,
            wind_speed: 1.36,
            wind_gust: Some(
                2.19,
            ),
            snow: None,
            rain: None,
            conditions: [
                WeatherCondition {
                    description: "nuageux",
                    id: BrokenClouds,
                },
            ],
        },
        // …
    ],
}
```

### [Web of Things](https://www.w3.org/WoT/)

To turn the `weather` program into standardized connected things, use
the `--into-thing` option: It will start a local Things server. The
`--thing-port` is useful to set the server's port.

Once the Things server is running, use a gateway like the [WebThings
Gateway](https://iot.mozilla.org/gateway/) to interact with the `weather`
thing. Enjoy!

```sh
$ /target/release/weather --openweathermap-api-key '<key>' --into-thing --thing-port 8086
Starting the Things server (port 8086)…
```
