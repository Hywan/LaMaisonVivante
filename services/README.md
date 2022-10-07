# Services

Services are files consumed by
[`systemctl`](https://en.wikipedia.org/wiki/Systemd).

## Install

Prior to the installation of the services, all `target/release/` bins must be present in `/usr/bin/`, a symlink is fine:

```sh
$ cd target/release
$ for i in {blinds,lights,hub-event-aggregator,hub-event-automator,lights,nilan,ui,victron,weather,kia} ; do sudo ln -s "$(pwd)/$i" /usr/bin/$i; done
```

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
