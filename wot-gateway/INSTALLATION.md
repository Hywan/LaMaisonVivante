# Installation

## Distribution

Use Raspberry Pi Imager to install the Raspbian Lite distribution.

Open the `boot` partition, and create an empty `ssh` file. In
addition, create a `wpa_supplicant.conf` file with the following
content:

```sh
country=fr
update_config=1
ctrl_interface=/var/run/wpa_supplicant

network={
 scan_ssid=1
 ssid="WiFi name"
 psk="WiFi password"
}
```

## Boot

Boot the Raspberry Pi. Find its IP, and connect with:

```sh
$ ssh pi@192.168.1.xxx
```

## Update

Once connected, run:

```sh
$ sudo apt update
$ sudo apt upgrade
```

## Change the root password

Run the following command:

```sh
$ sudo passwd
New password:
Retype new password:
```

## Zsh

To install Zsh, use the following command:

```sh
$ sudo apt install zsh
```

## Rust

We will use [`rustup`](https://rustup.rs/):


```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ export PATH="$HOME/.cargo/bin:$PATH"
```

## Git

To install Git, use the following command:

```sh
$ sudo apt install git
```

## LaMaisonVivante

To install various programs for our house:

```sh
$ sudo apt install libssl-dev
$ cd $HOME
$ mkdir development
$ git clone https://github.com/Hywan/LaMaisonVivante
$ cd LaMaisonVivante
```

Now build various programs by following the `README.md`s (likely with
`cargo build --release`).

## Mozilla IoT

To install the [Mozilla IoT
Gateway](https://github.com/mozilla-iot/gateway), first install
NodeJS and Python, then clone the repository and install it:

```sh
$ curl -sL https://deb.nodesource.com/setup_13.x | sudo bash -
$ sudo apt install nodejs
$ sudo apt install python3-pip
$ npm config set prefix $HOME
$ cd $HOME/development
$ git clone https://github.com/mozilla-iot/gateway mozilla-iot
$ cd mozilla-iot
$ sudo setcap cap_net_raw+eip $(eval readlink -f `which node`)
$ sudo setcap cap_net_raw+eip $(eval readlink -f `which python3`)
$ sudo apt install libboost-python-dev libboost-thread-dev libbluetooth-dev libglib2.0-dev
$ sudo apt install libusb-1.0-0-dev libudev-dev
$ sudo apt install autoconf
$ python3 -m pip install git+https://github.com/mozilla-iot/gateway-addon-python#egg=gateway_addon
$ npm ci
```

Finally, start it:

```sh
$ npm start
```
