# Hub Event Aggregator

The Event Aggregator is a very simple program that fetches data from
various WebThings, and saves them in [the
Database](../database). That's it!

## Installation

This program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```sh
$ cargo build --release
```

The executable binary is located in
`./target/release/hub-event-aggregator`.

## Usage

Use `-h`/`--help` to get help:

```
hub-event-aggregator 0.1.0

USAGE:
    hub-event-aggregator [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -c, --print-config-path    Prints the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -a, --addresses <addresses>...       Addresses to listen to, from which to collect and aggregate events, paired with
                                         their refresh rates, separated by a `@`, e.g. `localhost:1234@10`
    -d, --database-url <database-url>    The database URL
```

Use the `--addresses` option to specify the addresses of the WebThings
where to fetch the data from. The format is a bit specific: it's a
socket address, plus a timelapse, joined by an `@` sign.

Use the `--database-url` to connect to the database, something like :
`postgres://<user>:<password>@<host>/<database>` will be fine.

## Example

The following runs `hub-event-aggregator` by asking to fetch data from the WebThings such as:

* `127.0.0.1:8092` every 10 seconds,
* `127.0.0.1:8093` every 60 seconds…

… and to save all the fetched data in the database.

```sh
$ ./target/release/hub-event-aggregator \
      --addresses='127.0.0.1:8092@10' \
      --addresses='127.0.0.1:8093@60' \
      --database-url='postgres://user:password@host/database'
```
