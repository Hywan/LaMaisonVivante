#include <Controllino.h>

const int SECOND = 1000;

typedef struct {
  int button;
  int motor;
} Association;

const int NUMBER_OF_ASSOCIATIONS = 12;

const Association ASSOCIATIONS[NUMBER_OF_ASSOCIATIONS] = {
  // Kitchen, up
  {
   .button = CONTROLLINO_A3,
   .motor = CONTROLLINO_R2,
  },
  // Kitchen, down
  {
   .button = CONTROLLINO_A2,
   .motor = CONTROLLINO_R3,
  },
  // Living room, up
  {
   .button = CONTROLLINO_A0,
   .motor = CONTROLLINO_R0,
  },
  // Living room, down
  {
   .button = CONTROLLINO_A1,
   .motor = CONTROLLINO_R1,
  },
  // Parents' bedroom, up
  {
   .button = CONTROLLINO_A5,
   .motor = CONTROLLINO_R4,
  },
  // Parents' bedroom, down
  {
   .button = CONTROLLINO_A4,
   .motor = CONTROLLINO_R5,
  },
  // Éli's bedroom, up
  {
   .button = CONTROLLINO_A10,
   .motor = CONTROLLINO_R10,
  },
  // Éli's bedroom, down
  {
   .button = CONTROLLINO_A11,
   .motor = CONTROLLINO_R11,
  },
  // Louise's bedroom, up
  {
   .button = CONTROLLINO_A9,
   .motor = CONTROLLINO_R8,
  },
  // Louise's bedroom, down
  {
   .button = CONTROLLINO_A8,
   .motor = CONTROLLINO_R9,
  },
  // Bathroom, up
  {
   .button = CONTROLLINO_A6,
   .motor = CONTROLLINO_R6,
  },
  // Bathroom, down
  {
   .button = CONTROLLINO_A7,
   .motor = CONTROLLINO_R7,
  }
};

void setup() {
  for (int i = 0; i < NUMBER_OF_ASSOCIATIONS; ++i) {
    Association association = ASSOCIATIONS[i];

    pinMode(association.button, INPUT);
    pinMode(association.motor, OUTPUT);
  }
}

void loop() {
  for (int i = 0; i < NUMBER_OF_ASSOCIATIONS; ++i) {
    Association association = ASSOCIATIONS[i];

    //                              when the button is high…
    //         … the motor is high
    //                              when the button is low…
    //         … the motor is low
    digitalWrite(association.motor, digitalRead(association.button));
  }
}
