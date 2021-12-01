// To control this program remotely, use the `blinds-controller` program:
//
// ```sh
// $ blinds-controller --address 192.168.1.42:23 --subject livingroom --action closing
// ```
//
// Or alternatively, the hardcore way with `printf` and `netcat`:
//
// ```sh
// $ printf '%b\t%b' '\x02' '\x04' | nc 192.168.1.42 23 -v
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
const byte mac[] = {0x50, 0xD7, 0x53, 0x00, 0x05, 0x06};

// Use telnet port to send and receive data, because why not!
EthernetServer server = EthernetServer(23);

#define MS_TO_S(ms) (ms) / 1000

// Represents a pair (button, motor) respectively for the input and
// the output of the program.
typedef struct {
  // A button is a digital input with the following value: `HIGH` when
  // it's pushed, `LOW` when it's released.
  const int button;

  // A motor is actually a relay. “Motor” must be read as “the relay
  // that enables the motor”.
  const int motor;
} Control;

// Represents the possible states of a blind.
enum State: uint8_t {
  // The blind is not moving.
  Unmoving = 0,

  // The blind is moving up, just one step.
  Moving_Up = 1,

  // The blind is moving down, just one step.
  Moving_Down = 2,

  // The blind is opening, i.e. moving up until fully opened.
  Opening = 3,

  // The blind is closing, i.e. moving down until fully closed.
  Closing = 4,

  STATE_LAST,
};

// Represents a blind.
typedef struct {
  // The “up” control.
  const Control up;

  // The “down” control.
  const Control down;

  // Represents the time of the last event, which is more or less the
  // time where the state has possibly changed.
  unsigned long time_of_last_event;

  // Represents the state of the blind.
  State state;

  // Represents the time required to fully open or close the blind.
  const unsigned int trip_time;
} Blind;

// There are 6 blinds in our house.
const unsigned int NUMBER_OF_BLINDS = 6;

// Defines all the blinds.
static Blind BLINDS[NUMBER_OF_BLINDS] = {
  // Kitchen
  {
    .up = {
      .button = CONTROLLINO_A3,
      .motor = CONTROLLINO_R2,
    },
    .down = {
      .button = CONTROLLINO_A2,
      .motor = CONTROLLINO_R3,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 52,
  },
  // Living room
  {
    .up = {
      .button = CONTROLLINO_A0,
      .motor = CONTROLLINO_R0,
    },
    .down = {
      .button = CONTROLLINO_A1,
      .motor = CONTROLLINO_R1,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 52,
  },
  // Parents' bedroom
  {
    .up = {
      .button = CONTROLLINO_A4,
      .motor = CONTROLLINO_R5,
    },
    .down = {
      .button = CONTROLLINO_A5,
      .motor = CONTROLLINO_R4,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 40,
  },
  // Éli's bedroom
  {
    .up = {
      .button = CONTROLLINO_A10,
      .motor = CONTROLLINO_R10,
    },
    .down = {
      .button = CONTROLLINO_A11,
      .motor = CONTROLLINO_R11,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 40,
  },
  // Louise's bedroom
  {
    .up = {
      .button = CONTROLLINO_A9,
      .motor = CONTROLLINO_R8,
    },
    .down = {
      .button = CONTROLLINO_A8,
      .motor = CONTROLLINO_R9,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 40,
  },
  // Bathroom
  {
    .up = {
      .button = CONTROLLINO_A6,
      .motor = CONTROLLINO_R6,
    },
    .down = {
      .button = CONTROLLINO_A7,
      .motor = CONTROLLINO_R7,
    },
    .time_of_last_event = 0,
    .state = Unmoving,
    .trip_time = 40,
  },
};

// Represents a long press in seconds.
const unsigned int LONG_PRESS = 1;

// Initializes the system.
void setup() {
  Serial.begin(9600);

  for (unsigned int i = 0; i < NUMBER_OF_BLINDS; ++i) {
    Blind blind = BLINDS[i];

    pinMode(blind.up.button, INPUT);
    pinMode(blind.up.motor, OUTPUT);

    pinMode(blind.down.button, INPUT);
    pinMode(blind.down.motor, OUTPUT);
  }

  // Use the MAC address only, and let the DHCP server assign an IP
  // address.
  uint8_t retry_threshold = 5;
  uint8_t has_network = 0;

  while (retry_threshold > 0) {
    Serial.println(F("Trying to connect to the network"));

    if (Ethernet.begin(mac) == 0) {
      Serial.println(F("Failed to connect; will retry in 15 seconds"));

      retry_threshold--;
      delay(1000 * 15); // wait for 15 seconds.
    } else {
      Serial.println(F("Connected to the network successfully!"));

      has_network = 1;
      break;
    }
  }

  // No network…
  if (has_network == 0) {
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

    uint8_t blind_b = (uint8_t) bytes[0];
    uint8_t state_b = (uint8_t) bytes[2];

    // Invalid blind or state.
    if (blind_b >= NUMBER_OF_BLINDS || state_b >= STATE_LAST) {
      client.stop();

      return;
    }

    Blind *blind = &BLINDS[blind_b];
    State state = static_cast<State>(state_b);

    Serial.print(F("Blind: "));
    Serial.println(blind_b);

    Serial.print(F("State: "));
    Serial.println(state);

    unsigned long current_time = millis();

    blind->time_of_last_event = current_time;

    switch (state) {
      case Opening: {
        digitalWrite(blind->up.motor, HIGH);
        digitalWrite(blind->down.motor, LOW);
        blind->state = state;

        break;
      }

      case Closing: {
        digitalWrite(blind->up.motor, LOW);
        digitalWrite(blind->down.motor, HIGH);
        blind->state = state;

        break;
      }

      default:
        digitalWrite(blind->up.motor, LOW);
        digitalWrite(blind->down.motor, LOW);
        blind->state = Unmoving;
    }

    client.stop();

    return;
  }

  for (unsigned int i = 0; i < NUMBER_OF_BLINDS; ++i) {
    Blind *blind = &BLINDS[i];

    int up = digitalRead(blind->up.button);
    int down = digitalRead(blind->down.button);

    // Be sure `up` and `down` are mutually exclusive, i.e. it is not
    // possible to go up and down at the same time.
    // Even if physical buttons have a mechanical safety to avoid such
    // situations, and that the blind motors also have a safety
    // control for that, it's still better to add a new one.
    if (up == HIGH && down == HIGH) {
      digitalWrite(blind->up.motor, LOW);
      digitalWrite(blind->down.motor, LOW);
    } else {
      // Let's update the automata.
      switch (blind->state) {
        // The blind isn't moving. It is possible to move it up or
        // down.
        case Unmoving: {
          // Moving up the blind.
          if (up == HIGH) {
            digitalWrite(blind->up.motor, HIGH);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Moving_Up;
          }
          // Moving down the blind.
          else if (down == HIGH) {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, HIGH);
            blind->state = Moving_Down;
          }
          // No button is pressed. Reset the controls for the sake of
          // safety and stay in the same state.
          else {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Unmoving;
          }

          blind->time_of_last_event = millis();

          break;
        }

        // The blind is in the `Moving_Up` state. It is possible to
        // keep pressing the `up` button, until a long pressed is
        // triggered, or to calibrate the blind.
        case Moving_Up: {
          // Continue to move the blind up.
          if (up == HIGH) {
            unsigned long current_time = millis();

            // `millis()` can overflow and thus can go back to zero.
            if (current_time < blind->time_of_last_event) {
              // Reset the `time_of_last_event`.
              blind->time_of_last_event = current_time;
            }
            // `up` is “long pressed”. Move to the `Opening` state.
            else if (MS_TO_S(current_time - blind->time_of_last_event) >= LONG_PRESS) {
              blind->state = Opening;
              blind->time_of_last_event = current_time;
            }
          }
          // The `up` button is released, or the `down` button is
          // pressed. Both have the same consequences: Stop moving up,
          // and move to the `Unmoving` state.
          else {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Unmoving;
            blind->time_of_last_event = millis();
          }

          break;
        }

        // The blind is in the `Moving_Down` state. It is possible to
        // keep pressing the `down` button, until a long pressed is
        // triggered, or to calibrate the blind.
        case Moving_Down: {
          // Continue to move the blind down.
          if (down == HIGH) {
            unsigned long current_time = millis();

            // `millis()` can overflow and thus can go back to zero.
            if (current_time < blind->time_of_last_event) {
              // Reset the `time_of_last_event`.
              blind->time_of_last_event = current_time;
            }
            // `down` is “long pressed”. Move to the `Closing` state.
            else if (MS_TO_S(current_time - blind->time_of_last_event) >= LONG_PRESS) {
              blind->state = Closing;
              blind->time_of_last_event = current_time;
            }
          }
          // The `down` button is released, or the `up` button is
          // pressed. Both have the same consequences: Stop moving
          // down, and move to the `Unmoving` state.
          else {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Unmoving;
          }

          break;
        }

        // The blind is in the `Opening` state. It is possible to stop
        // it manually by pressing the `down` button, or to wait the
        // necessary trip time.
        case Opening: {
          unsigned long current_time = millis();

          if (
              // `millis()` can overflow and thus can go back to
              // zero. Match this edge case behavior as a “stop”.
              (current_time < blind->time_of_last_event) ||

              // It should be fully opened now.
              (MS_TO_S(current_time - blind->time_of_last_event) >= blind->trip_time) ||

              // `down` is pressed to cancel the opening.
              (down == HIGH)
          ) {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Unmoving;
            blind->time_of_last_event = current_time;
          }

          break;
        }

        // The blind is in the `Closing` state. It is possible to stop
        // it manually by pressing the `up` button, or to wait the
        // necessary trip time.
        case Closing: {
          unsigned long current_time = millis();

          if (
              // `millis()` can overflow and thus can go back to
              // zero. Match this edge case behavior as a “stop”.
              (current_time < blind->time_of_last_event) ||

              // It should be fully opened now.
              (MS_TO_S(current_time - blind->time_of_last_event) >= blind->trip_time) ||

              // `up` is pressed to cancel the opening.
              (up == HIGH)
          ) {
            digitalWrite(blind->up.motor, LOW);
            digitalWrite(blind->down.motor, LOW);
            blind->state = Unmoving;
            blind->time_of_last_event = current_time;
          }

          break;
        }

        // Unreachable. For the sake of safety.
        default: {
          digitalWrite(blind->up.motor, LOW);
          digitalWrite(blind->down.motor, LOW);
        }
      }
    }
  }
}

// Local Variables:
// mode: c
// End:
// vim: set ft=c :
