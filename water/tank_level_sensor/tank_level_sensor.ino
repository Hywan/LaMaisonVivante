#include <SPI.h>
#include <WiFiNINA.h>
#include "tank_level_secrets.h"

// HC-SR04 has 2 pins (in addition to Vcc and Ground): Trig and Echo.
// It is connceted to an Arduino Nano 33 IoT.
//
// In our context, Trig is connected to D5, and Echo is connected to D6.
const int trigger_pin = 5; 
const int echo_pin = 6;

// Use HTTP port to send and receive data.
WiFiServer server = WiFiServer(80);

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

  if (WiFi.firmwareVersion() < WIFI_FIRMWARE_LATEST_VERSION) {
    Serial.print("Firmware is outdated, need to update it.");

    while(true);
  }

  int wifi_status = WL_IDLE_STATUS;

  while (wifi_status != WL_CONNECTED) {
    Serial.print(F("Try to connect to WiFi network: "));
    Serial.println(WIFI_SSID);

    wifi_status = WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

    delay(5000);
  }

  server.begin();

  Serial.println(F("Connected to WiFi"));
  Serial.println(F("Setup OK!"));
}

void loop() {
  WiFiClient client = server.available();

  if (client && client.connected()) {
    Serial.println(F("New client"));

    float distances[3] = {0.0, 0.0, 0.0};

    distances[0] = compute_distance();
    delay(500);

    distances[1] = compute_distance();
    delay(500);

    distances[2] = compute_distance();

    Serial.println(F("Computed distances: "));
    Serial.println(distances[0]);
    Serial.println(distances[1]);
    Serial.println(distances[2]);

    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: application/json");
    client.println("Connection: close");
    client.println();
    client.print("{\"distance\": [");
    client.print(distances[0]);
    client.print(", ");
    client.print(distances[1]);
    client.print(", ");
    client.print(distances[1]);
    client.println("]}");

    delay(100);
    client.stop();

    Serial.println(F("Client disconnected"));
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
