# Hub UI

The UI is a Web application that can be installed natively or run in a
browser. It can be understood as a WebThing gateway. It provides a
nice real-time overview of several WebThings, like the battery, the
solar PV, the ventilation, the domestic hot water, the humidity
etc. It also adds actions to control several things, like the lights
and the blinds.

The stack is as small as possible. Thus, there is no JavaScript or CSS
framework. It's plain HTML, vanilla JavaScript and simple CSS.

Everything is developed for my specific use cases for the moment.

## Installation

Nothing to do. (Yup).

## Usage

Open `index.html` and enjoy. Of course, the other WebThings programs
must run, otherwise no data can be fetched.

## Screenshots

It looks like this.

<table>
  <thead>
   <tr>
    <th>Home</th>
    <th colspan="2">Realtime Metrics</th>
    <th>Daily Metrics</th>
   </tr>
  </thead>
  <tbody>
   <tr valign="top">
    <td><img src="./doc/home.png" alt="UI: Home" /></td>
    <td><img src="./doc/metrics_realtime_part1.png" alt="UI: Real time metrics (1/2)" /></td>
    <td><img src="./doc/metrics_realtime_part2.png" alt="UI: Real time metrics (2/2)" /></td>
    <td><img src="./doc/metrics_daily.png" alt="UI: Daily metrics" /></td>
   </tr>
  </tbody>
</table>
