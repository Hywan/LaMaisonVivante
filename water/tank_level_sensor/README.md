# Water Tank Level

An [Arduino Nano 33 IoT](https://store.arduino.cc/arduino-nano-33-iot)
is connected to an [HC-SR04 Ultrasonic Sonar Distance
Sensor](https://www.adafruit.com/product/3942).

The `tank_level_sensor.ino` program is the one that runs in the
Arduino. Once the Arduino Nano 33 IoT is running, it will
automatically connect to a WiFi network (WPA Personal
encryption). Immediately, a primitive HTTP server will run. Getting
`/` will reply with a JSON payload representing the distance of the
water in the tank (or any distance to a surface).

```sh
$ curl 192.168.1.42 | python -m json.tool
{
    "average_distance": 153.42,
    "number_of_samples": 5
}
```

The Arduino turns into deep sleep for 5 seconds everytime no client is
connected, in order to save the battery.
