# Blinds

In our house, blinds are directly plugged in a
[Controllino](https://www.controllino.biz/). It means that physical
buttons are inputs of the Controllino.

Hence, blinds are fully controlled by the `blinds.ino` Arduino
program, that lands in the Controllino. It sets a simple automata to
control the motor of the blinds. The code is largely documented, so
enjoy!

It also supports a remote control over TCP. It sets up a TCP server and a specific but very basic binary protocol to control the blinds. It assumes the Controllino is wired by RJ45 to a router with a DHCP server. Thus, the Controllino IP is assigned dynamically by DHCP. The `blinds.ino` program uses the
by DHCP. The `lights.ino` program uses the
[Telnet](https://en.wikipedia.org/wiki/Telnet) port (23) to receive
data (because why not 🤷‍♂️).
Example of raw usage with [`netcat`](https://nc110.sourceforge.io/):

```sh
$ printf '%b\t%b' '\x02' '\x04' | nc 192.168.1.42 23 -v
#                  ^~~~   ^~~~       ^~~~~~~~~~~~ ^~
#                  |      |          |            |
#                  |      |          |            the port
#                  |      |          the IP
#                  |      the action
#                  the subject
```

_Hopefully_, there is a [Rust](https://www.rust-lang.org/) program to
control the blinds! Please, welcome `blinds-controller`. Here is a
basic usage, but check [its documentation to learn
more](blinds-controller/).

```sh
$ blinds-controller --address 192.168.1.42:23 --subject livingroom --action closing
```

