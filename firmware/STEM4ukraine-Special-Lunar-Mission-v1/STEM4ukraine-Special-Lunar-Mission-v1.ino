/*
 *
 *   Copyright (C) 2024 STEM4ukraine
 *   Website https://github.com/STEM4ukraine
 *   
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License at <http://www.gnu.org/licenses/> 
 *   for more details.
 *
 */


// this uses https://github.com/MCUdude/MicroCore
// which can be accessed in the boards manager by adding
//  https://mcudude.github.io/MicroCore/package_MCUdude_MicroCore_index.json

// the code has been uploaded using an old USB-ASP programmer clone
// and the "USB-ASP slow (Microcore)" setting in the arduino IDE
// uses 1.2MHz internal oscillator on attiny13

#define DITMS 900
#define DAHMS DITMS*3

#define LED1 0b00001000
#define LED2 0b00010000
#define LED3 0b00000100
#define LED4 0b00000001

#define LFSR_INIT 0xfeed // non-zero seed value for Galois LFSR used to generate the white noise
#define LFSR_MASK 0x8016 // polynomial from http://users.ece.cmu.edu/~koopman/lfsr/


uint8_t button = 0;

void setup() {
  pinMode(0, OUTPUT); // LED4
  pinMode(1, OUTPUT); // PIEZO
  pinMode(2, OUTPUT); // LED3
  pinMode(3, OUTPUT); // LED1
  pinMode(4, OUTPUT); // LED2
  pinMode(5, INPUT);  // SW1
}

byte poll_button() {
  button = button|(analogRead(A0) < 950);
  return button;
}

void delay_with_polling(uint16_t delay_ms) {
  int i;
  int increment = delay_ms/10;
  for (i = 0; i < delay_ms; i=i+increment) {
    delay(increment);
    if (poll_button) break;
  }
}

// note period increase from note_per to (note_per + 240) during call;
// with four consecutive calls, note_per can begin at 330 and finish at 1290 period
void play_descent(uint16_t note_per) {
  uint8_t i,k;
  uint16_t cycles, new_note;

  for (i = 0; i < 48; i++) {
    new_note = note_per+i*5;
    cycles = 16000/new_note; // gives constant duration per note
    while (cycles > 0) {
      PORTB |= (1 << PB1);
      delayMicroseconds(new_note);
 
      PORTB &= ~(1 << PB1);
      delayMicroseconds(new_note);
      cycles--;
    }
  }
}

void play_landing(uint16_t cycles) {
  uint8_t k,i;

  static uint16_t lfsr = LFSR_INIT; 

  for (i = cycles; i > 0; i--) {                            
    for (k = cycles; k > 0; k--) {                            
     if(lfsr & 1) {
       lfsr =  (lfsr >>1) ^ LFSR_MASK;
       PORTB |= (1 << PB1);
     } else {
       lfsr >>= 1;
       PORTB &= ~(1 << PB1);
     }
     delayMicroseconds(50);
   }
  }

}

void led_on(uint16_t duration) {
  PORTB ^= LED1;
  delay_with_polling(duration);
}

void led_off(uint16_t duration) {
  PORTB ^= LED1;
  delay_with_polling(duration);
}

void emergency_beacon() {
  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DITMS);
  led_on(DAHMS);
  led_off(DAHMS);

  led_on(DAHMS);
  led_off(DAHMS);

  led_on(DITMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);

  led_on(DAHMS);
  led_off(DITMS);
  led_on(DITMS);
  led_off(DAHMS);
}

// main loop polls the button and
// starts special lunar operation
// if the button has been pressed
void loop() {

  button = 0;

  while(1) {
    if (poll_button()) {
      button = 0;
      play_descent(330); //690
      PORTB ^= LED4; // on
      play_descent(570); //810
      PORTB ^= LED4; // off
      PORTB ^= LED3; // on
      play_descent(810); //930
      PORTB ^= LED3; // off
      PORTB ^= LED2; // on
      play_descent(1050); //1050
      PORTB ^= LED2; // off
      PORTB ^= LED1; // on
      play_landing(180);
      PORTB ^= LED1; // off
      delay(1000);
      emergency_beacon();
    }
  }
}
