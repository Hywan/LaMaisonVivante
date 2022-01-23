# Installation

## Distribution

Use Raspberry Pi Imager to install the Raspbian Lite distribution.

Open the `boot` partition, and create an empty `ssh` file. In
addition, create a `wpa_supplicant.conf` file with the following
content:

```
country=fr
update_config=1
ctrl_interface=/var/run/wpa_supplicant

network={
 scan_ssid=1
 ssid="WiFi name"
 psk="WiFi password"
}
```

Edit the `config.txt` file to add:

```
arm_64bit=1
```

to boot Raspbian in 64-bit mode ([learn
more](https://www.raspberrypi.org/forums/viewtopic.php?t=250730)).

## Boot from SSH

(https://yves.io/blog/2020/08/251736/_/how-to-boot-a-raspberry-pi-4-from-ssd-the-ultimate-2020-guide)

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

## Python

To install Python, use the following commands:

```sh
$ sudo apt install python3-pip
$ pip3 install virtualenv
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

## Update timezone

To update the local time:

```sh
$ sudo timedatectl set-timezone Europe/Zurich
```
