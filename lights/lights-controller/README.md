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
lights-controller 0.1.0

USAGE:
    lights-controller [OPTIONS] --address <address> --subject <subject>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -x, --action <action>      Type of signal/event to send on the light [default: pulse]  [possible values: Pulse]
    -a, --address <address>    Address of the Controllino; see `lights.ino` to see the port; e.g. `192.168.1.42:23`
    -s, --subject <subject>    Light to control [possible values: LaundryRoom, Bathroom, LouiseBedroom, EliBedroom,
                               Hall, LivingRoom, SittingRoom, DiningTable, KitchenIsland, Kitchen, ParentBed,
                               ParentBathroom, ParentBedroom]
```

Use the `--address` option to specify the address, and the `--subject`
option to specify the group of lights to control. The `--action`
option defaults to `pulse`, which is also the only possible value for
the moment, so you can skip it.

## Example

To turn the group of lights in the living room (a set of 5 lights):

```sh
$ /target/release/lights-controller -a 192.168.1.125:23 -s livingroom
Sending a Pulse to LivingRoom…
```
