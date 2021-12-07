# Services

Services are files consumed by
[`systemctl`](https://en.wikipedia.org/wiki/Systemd).

## Install

They must be installed in `/etc/systemd/system`.

## Run

To check a service is active, or to check its status:

```sh
$ systemctl is-active <name>
$ systemctl status <name>
```

To start a service:

```sh
$ sudo systemctl start <name>
```

To stop a service:

```sh
$ sudo systemctl stop <name>
```

To allow the service to start at boot time, run:

```sh
$ sudo systemctl enable <name>
```
