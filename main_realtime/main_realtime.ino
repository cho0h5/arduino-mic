void setup() {
  Serial.begin(115200);
  
  OCR0A = 0xF9; // 249
  TCCR0B = (1 << WGM01) | (1 << CS01);
  TIMSK0 = (1 << OCIE1A);

  initADC();
}

void loop() {
}

ISR(TIMER0_COMPA_vect) {  // 8000hz
  Serial.write(readADC());
}

void initADC() {
  ADMUX |= (1 << REFS0) | (1 << ADLAR); // ADLAR -> ADCH, ADCL 왼쪽 정렬
}

byte readADC() {
  ADCSRA |= (1 << ADSC);  // start conversion
  while(ADCSRA & (1 << ADIF));  // interrupt flag <- when conversion completed

  uint8_t high;
  high = ADCH;  // read only 1 byte

  return high;
}
