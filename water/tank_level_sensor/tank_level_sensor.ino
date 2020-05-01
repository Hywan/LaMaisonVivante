// Once the Arduino Nano 33 IoT is running, it will automatically
// connect to a WiFi network (WPA Personal encryption). Immediately, a
// primitive HTTP server will run. Getting `/` will reply with a JSON
// payload representing the distance of the water in the tank (or any
// distance to a surface).
//
// ```sh
// $ curl 192.168.1.42 | python -m json.tool
// {
//     "average_distance": 153.42,
//     "number_of_try": 5
// }
// ```

#include <SPI.h>
#include <WiFiNINA.h>

// Set up WiFi data.
#include "tank_level_secrets.h"

// HC-SR04 has 2 pins (in addition to Vcc and Ground): Trig and Echo.
// It is connected to an Arduino Nano 33 IoT.
//
// In our context, Trig is connected to D5, and Echo is connected to D6.
const int trigger_pin = 5; 
const int echo_pin = 6;

// Use HTTP port to send and receive data.
WiFiServer server = WiFiServer(80);

// Number of try when calculating the distance.
const uint8_t NUMBER_OF_TRY = 5;

void setup() {
  Serial.begin(9600);

  while (!Serial) {}

  // Set up pins for the HC-SR04 sensor.
  pinMode(trigger_pin, OUTPUT); 
  pinMode(echo_pin, INPUT);

  // Set up the WiFi.
  if (WiFi.status() == WL_NO_MODULE) {
    Serial.println(F("WiFi failed to connect."));

    while(true);
  }

  if (strcmp(WiFi.firmwareVersion(), WIFI_FIRMWARE_LATEST_VERSION) < 0) {
    Serial.print(F("Firmware is outdated, need to update it."));

    while(true);
  }

  int wifi_status = WL_IDLE_STATUS;

  // Connect to the WiFi network.
  while (wifi_status != WL_CONNECTED) {
    Serial.print(F("Try to connect to WiFi network: "));
    Serial.println(WIFI_SSID);

    wifi_status = WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

    delay(5000);
  }

  server.begin();

  Serial.println(F("Connected to WiFi!"));
  Serial.println(F("Setup OK!"));
}

void loop() {
  WiFiClient client = server.available();

  // A new client connects.
  if (client && client.connected()) {
    Serial.println(F("New client."));
    Serial.println(F("Computed distances: "));

    // Gather some distances.
    float distances[NUMBER_OF_TRY];

    for (uint8_t i = 0; i < NUMBER_OF_TRY; ++i) {
      distances[i] = compute_distance();
      Serial.println(distances[i]);

      delay(500);
    }

    // Compute the average.
    float average_distance = 0.0;

    for (uint8_t i = 0; i < NUMBER_OF_TRY; ++i) {
      average_distance += distances[i];
    }

    average_distance = average_distance / NUMBER_OF_TRY;

    // Write the response, as an JSON payload.
    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: application/json");
    client.println("Connection: close");
    client.println();
    client.print("{\"average_distance\": ");
    client.print(average_distance);
    client.print(", \"number_of_try\": ");
    client.print(NUMBER_OF_TRY);
    client.println("}");

    delay(100);
    client.stop();

    Serial.println(F("Client disconnected."));
  }
}

float compute_distance() {
  // Ensure to reset the trigger pin.
  digitalWrite(trigger_pin, LOW); 
  delayMicroseconds(2);

  // Trigger the pulse for 10µs. It sends out 8 sound waves of 40KHz.
  digitalWrite(trigger_pin, HIGH); 
  delayMicroseconds(10);
  digitalWrite(trigger_pin, LOW);

  // Read the echo pin to calculate the time taken by the sound waves
  // to come back.
  float duration = pulseIn(echo_pin, HIGH);

  // Calculate the distance based on the duration.
  float distance = (duration * .0343) / 2; 

  return distance;
}

// Local Variables:
// mode: c
// End:
// vim: set ft=c :
