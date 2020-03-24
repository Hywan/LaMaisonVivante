# Lights

In our house, lights are behind [latching
switches](https://en.wikipedia.org/wiki/Latching_switch). Then, any
button in the house can change the state of any lights, but a
microcontroller can do the same! A
[Controllino](https://www.controllino.biz/) is positionned as inputs
of the latching switches, just like regular buttons.

The `lights.ino` Arduino program lands in the Controllino. It sets up
a TCP server and a specific but very basic binary protocol to control
the lights. It assumes the Controllino is wired by RJ45 to a router
with a DHCP server. Thus, the Controllino IP is assigned dynamically
by DHCP. The `lights.ino` program uses the
[Telnet](https://en.wikipedia.org/wiki/Telnet) port (23) to receive
data (because why not 🤷‍♂️).

Example of raw usage with [`netcat`](https://nc110.sourceforge.io/):

```sh
$ printf '%b\t%b' '\x05' '\x00' | nc 192.168.1.42 23 -v
#                  ^~~~   ^~~~       ^~~~~~~~~~~~ ^~
#                  |      |          |            |
#                  |      |          |            the port
#                  |      |          the IP of the Controllino
#                  |      the action
#                  the subject
```

_Hopefully_, there is a [Rust](https://www.rust-lang.org/) program to
control the lights! Please, welcome `lights-controller`. Here is a
basic usage, but check [its documentation to learn
more](lights-controller/).

```sh
$ lights-controller --address 192.168.1.42:23 --subject livingroom
```

