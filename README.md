# Home Automation programs for [_La Maison Vivante_](https://lamaisonvivante.blog/)

Some programs used to automate our house. The house is entirely
self-sufficient, which means it is out-of-grid for water and
electricity. There is also no central heating system. And for the most
curious of you, dear readers, the house is made of straw, wood, and
earth, while being super modern! But this repository is about programs
that are used for the home automation, monitoring etc.

# Navigation

## Blinds

Blinds are controlled by the `blinds.ino` program, that lands in a
Controllino. [Learn more](blinds/).

## Domestic Hot Water/Ventilation

Supplied air, extracted air, temperatures, CO<sub>2</sub>, Domestic
Hot Water (DHW), Storage Hot Water (SHW) etc. are monitored with the
`nilan-reader` program. [Learn more](dhw-ventilation/nilan-reader/).

## Electricity

Batteries, PV inverter, and house are monitored with the
`victron-reader` program. [Learn more](electricity/victron-reader/).

## Lights

Lights are controlled by the `lights.ino` program, that lands in a
Controllino, along with its companion `lights-controller`. [Learn
more](lights/).
