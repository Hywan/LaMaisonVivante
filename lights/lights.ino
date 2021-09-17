// To control this program, use the `lights-controller` program:
//
// ```sh
// $ lights-controller --address 192.168.1.42:23 --subject livingroom --action pulse
// ```
//
// Or alternatively, the hardcore way with `printf` and `netcat`:
//
// ```sh
// $ printf '%b\t%b' '\x05' '\x00' | nc 192.168.1.42 23 -v
// #                  ^~~~   ^~~~       ^~~~~~~~~~~~ ^~
// #                  |      |          |            |
// #                  |      |          |            the port
// #                  |      |          the IP
// #                  |      the action
// #                  the subject
// ```

#include <Controllino.h>
#include <Ethernet.h>

// A MAC address for the Controllino.
const byte mac[] = {0x50, 0xD7, 0x53, 0x00, 0x05, 0x05};

// Use telnet port to send and receive data, because why not!
EthernetServer server = EthernetServer(23);

// Represents a subject that can receive an action.
enum Subject: uint8_t {
  LaundryRoom = 0,
  Bathroom = 1,
  LouiseBedroom = 2,
  EliBedroom = 3,
  Hall = 4,
  LivingRoom = 5,
  SittingRoom = 6,
  DiningTable = 7,
  KitchenIsland = 8,
  Kitchen = 9,
  ParentBed = 10,
  ParentBathroom = 11,
  ParentBedroom = 12,
  GreenHouse = 13,
  SUBJECT_LAST,
};

// Represents an action for a subject.
enum Action: uint8_t {
  Pulse = 0,
  ACTION_LAST,
};

// Number of lights. It is the length of subjects since all subjects
// are lights in this context.
const unsigned int NUMBER_OF_LIGHTS = SUBJECT_LAST;

// Defines all lights.
static Subject LIGHTS[NUMBER_OF_LIGHTS] = {
  // Laundry room
  CONTROLLINO_R14,

  // Bathroom
  CONTROLLINO_R12,

  // Louise's bedroom
  CONTROLLINO_R15,

  // Éli's bedroom
  CONTROLLINO_R13,

  // Hall
  CONTROLLINO_R11,

  // Living room
  CONTROLLINO_R10,

  // Sitting room
  CONTROLLINO_R6,

  // Dining table
  CONTROLLINO_R7,

  // Kitchen island
  CONTROLLINO_R3,

  // Kitchen
  CONTROLLINO_R4,

  // Parents' bed
  CONTROLLINO_R0,

  // Parents' bathroom
  CONTROLLINO_R2,

  // Parents' bedroom
  CONTROLLINO_R1,

  // Green house
  CONTROLLINO_R8,
};

// Initializes the system.
void setup() {
  Serial.begin(9600);

  for (unsigned int i = 0; i < NUMBER_OF_LIGHTS; ++i) {
    pinMode(LIGHTS[i], OUTPUT);
  }

  // Use the MAC address only, and let the DHCP server assign an IP
  // address.
  if (Ethernet.begin(mac) == 0) {
    Serial.println(F("No IP assigned by the DHCP server"));

    for (;;)
      ;
  }

  Serial.print(F("Local IP: "));
  Serial.println(Ethernet.localIP());
}

// Here we are.
void loop() {
  EthernetClient client = server.available();

  if (client) {
    Serial.println(F("New connection"));

    char bytes[] = {0, 0, 0};
    size_t result = client.readBytes(bytes, 3);

    // Invalid payload.
    if (result < 3) {
      client.stop();

      return;
    }

    // Invalid separator.
    if (bytes[1] != '\t') {
      client.stop();

      return;
    }

    uint8_t subject_b = (uint8_t) bytes[0];
    uint8_t action_b = (uint8_t) bytes[2];

    // Invalid subject or action.
    if (subject_b >= SUBJECT_LAST || action_b >= ACTION_LAST) {
      client.stop();

      return;
    }

    Subject subject = static_cast<Subject>(subject_b);
    Action action = static_cast<Action>(action_b);

    Serial.print(F("Subject: "));
    Serial.println(subject);

    Serial.print(F("Action: "));
    Serial.println(action);

    switch (action) {
      case Pulse:
        digitalWrite(LIGHTS[subject], HIGH);
        delay(100);
        digitalWrite(LIGHTS[subject], LOW);

        break;
    }

    client.stop();
  }
}

// Local Variables:
// mode: c
// End:
// vim: set ft=c :
