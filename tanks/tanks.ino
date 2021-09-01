

#include <WiFi.h>

// Set up WiFi data.
#include "tanks_secrets.h"

// HC-SR04 has 2 pins (in addition to Vcc and Ground): Trig and Echo.
// It is connected to an ESP32.
//
// In our context, Trig is connected to D16, and Echo is connected to D17.
const int trigger_pin = 16; 
const int echo_pin = 17;

// The ESP32 sends measures to a server. Let's define its port.
const uint16_t SERVER_PORT = 1234;

// Number of samples when calculating the distance.
const uint8_t NUMBER_OF_SAMPLES = 5;

// Factor to convert second.
#define S_TO_uS 1000000ULL

// Deep sleep duration (in seconds).
const uint16_t TIME_TO_SLEEP = 60 * 60 * 12;

RTC_DATA_ATTR int number_of_runs = 0;

void setup() {
  Serial.begin(9600);

  delay(2000);

  Serial.println(F("\n\nStarting"));

  ++number_of_runs;

  Serial.print(F("Run number: "));
  Serial.println(number_of_runs);

  // Set up pins for the HC-SR04 sensor.
  pinMode(trigger_pin, OUTPUT);
  pinMode(echo_pin, INPUT);

  // Set up the WiFi.
  int wifi_status = WL_IDLE_STATUS;

  // Connect to the WiFi network.
  while (wifi_status != WL_CONNECTED) {
    Serial.print(F("Try to connect to WiFi network: "));
    Serial.println(WIFI_SSID);

    wifi_status = WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

    delay(5000);
  }

  Serial.println(F("Connected to WiFi!"));
  Serial.print(F("WiFi SSID: "));
  Serial.println(WiFi.SSID());

  Serial.print(F("WiFi IP Address: "));
  Serial.println(WiFi.localIP());;

  Serial.print(F("WiFi signal strength (RSSI): "));
  Serial.print(WiFi.RSSI());
  Serial.println(F(" dBm"));

  Serial.println(F("Try to connect the client"));

  IPAddress server(192, 168, 1, 128);
  WiFiClient client;

  if (client.connect(server, SERVER_PORT)) {
    Serial.println(F("Setup OK!"));

    Serial.println(F("Computing distances"));

    // Gather some distances.
    float distances[NUMBER_OF_SAMPLES];

    for (uint8_t i = 0; i < NUMBER_OF_SAMPLES; ++i) {
      distances[i] = compute_distance();
      Serial.println(distances[i]);

      delay(500);
    }

    // Compute the average.
    float average_distance = 0.0;

    for (uint8_t i = 0; i < NUMBER_OF_SAMPLES; ++i) {
      average_distance += distances[i];
    }

    average_distance = average_distance / NUMBER_OF_SAMPLES;

    Serial.println(F("Sending data"));

    // Write the response, as an JSON payload.
    client.print(F("{\"average_distance\": "));
    client.print(average_distance);
    client.print(F(", \"number_of_samples\": "));
    client.print(NUMBER_OF_SAMPLES);
    client.print(F(", \"number_of_runs\": "));
    client.print(number_of_runs);
    client.println(F("}"));
    client.println();
    client.flush();

    // Disconnecting the client.
    Serial.println(F("Disconnecting client"));
    delay(2000);
    client.stop();
  }

  // Disconnecting and disabling WiFi.
  Serial.println(F("Disconnecting WiFi"));
  WiFi.disconnect();

  // Configure the wake up source.
  esp_sleep_enable_timer_wakeup(TIME_TO_SLEEP * S_TO_uS);

  // Sleep.
  Serial.println(F("Going to deep sleep now"));
  Serial.flush();
  esp_deep_sleep_start();
}

void loop() {
  // Never executed.
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
