# Lights Controller

This program aims at controlling the lights in our house, by talking
to the `lights.ino` program that lands in a Controllino. To learn more
about how it works, see [the parent documentation](../).

## Installation

This program is written in [Rust](https://www.rust-lang.org/). Just
clone the program, and run:

```sh
$ cargo build --release
```

The executable binary is located in
`./target/release/lights-controller`.

## Usage

Use `-h`/`--help` to get help:

```
lights-controller 0.3.0

USAGE:
    lights-controller [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                 Prints help information
    -t, --into-thing           Turns this program into a Thing, i.e. a new Web of Things device
    -c, --print-config-path    Prints the configuration path and exit
    -V, --version              Prints version information

OPTIONS:
    -x, --action <action>            Type of signal/event to send on the light [default: Pulse]  [possible values:
                                     Pulse]
    -a, --address <address>          Address of the Controllino; see `lights.ino` to see the port; e.g.
                                     `192.168.1.42:23`. This option overwrites the value read from the configuration
                                     file
    -s, --subject <subject>          Light to control [default: LivingRoom]  [possible values: LaundryRoom, Bathroom,
                                     LouiseBedroom, EliBedroom, Hall, LivingRoom, SittingRoom, DiningTable,
                                     KitchenIsland, Kitchen, ParentBed, ParentBathroom, ParentBedroom]
    -p, --thing-port <thing-port>    Port of the Thing. Requires `--into-thing` to be effective. This option overwrites
                                     the value read from the configuration file
```

Use the `--address` option to specify the address, and the `--subject`
option to specify the group of lights to control. The `--action`
option defaults to `pulse`, which is also the only possible value for
the moment, so you can skip it.

A configuration file can be used to read the value of the `--address`
option. Use `--print-config-path` to get the path to the configuration
file.

## Example

### Basic usage

To turn the group of lights in the living room (a set of 5 lights):

```sh
$ /target/release/lights-controller -a 192.168.1.125:23 -s livingroom
Sending a Pulse to LivingRoom…
```

### [Web of Things](https://www.w3.org/WoT/)

To turn all the lights into standardized connected things, use the
`--into-thing` option: It will start a local Things server. The
`--thing-port` is useful to set the server's port.

Once the Things server is running, use a gateway like the [WebThings
Gateway](https://iot.mozilla.org/gateway/) to interact with the
lights. Enjoy!

```sh
$ ./target/release/lights-controller --address 192.168.1.125:23 --into-thing --thing-port 8081
Starting the Things server (port 8081)…
```
