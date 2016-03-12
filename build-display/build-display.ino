#include <Adafruit_TFTLCD.h>
#include <Adafruit_GFX.h>
#include <pin_magic_UNO.h>
#include <registers.h>

// The control pins for the LCD can be assigned to any digital or
// analog pins...but we'll use the analog pins as this allows us to
// double up the pins with the touch screen (see the TFT paint example).
#define LCD_CS A3 // Chip Select goes to Analog 3
#define LCD_CD A2 // Command/Data goes to Analog 2
#define LCD_WR A1 // LCD Write goes to Analog 1
#define LCD_RD A0 // LCD Read goes to Analog 0
#define LCD_RESET A4 // Can alternately just connect to Arduino's reset pin

// Pins for the LCD Shield
#define YP A3 // must be analog
#define XM A2 // must be analog
#define YM 9  // digital or analog pin
#define XP 8  // digital or analog pin

#define BLACK   0x0000
#define BLUE    0x001F
#define RED     0xF800
#define GREEN   0x07E0
#define CYAN    0x07FF
#define MAGENTA 0xF81F
#define YELLOW  0xFFE0
#define WHITE   0xFFFF

Adafruit_TFTLCD tft(LCD_CS, LCD_CD, LCD_WR, LCD_RD, LCD_RESET);

void setup() {
  Serial.begin(9600);
  while (!Serial) {
    ; // wait for port to connect
  }
  pinMode(XM, OUTPUT);
  pinMode(YP, OUTPUT);
  tft.reset();

  uint16_t identifier = tft.readID();

  tft.begin(identifier);
  tft.setRotation(1);
  establishContact();
}

void drawButtons() {
  tft.fillCircle(305, 20, 10, MAGENTA);
  tft.fillTriangle(295, 60, 305, 40, 315, 60, GREEN);
  tft.fillRect(295, 75, 20, 15, BLUE);
}

void establishContact() {
  tft.fillScreen(BLACK);
  tft.setTextColor(RED);
  tft.setCursor(0, 0);
  tft.setTextSize(2);
  tft.println("Connecting...");
// TODO - this creates an infinite loop
  //while(Serial.available() <= 0) {
    Serial.print("START");
    delay(300);
  //}
}

int state = 0;

void loop() {
  tft.fillScreen(BLACK);
  tft.setTextColor(BLUE);
  tft.setCursor(0, 0);
  tft.setTextSize(2);
  tft.println("GitLab Build Status\n");
  drawButtons();
  tft.setTextSize(2);
  while (Serial.available() > 0) {
    String message = Serial.readStringUntil(';');
    if (state == 0) { // header
      tft.setTextColor(MAGENTA);
      tft.println("Project:");
      tft.println(" " + message);
      tft.println();
      tft.setTextColor(CYAN);
      tft.println("Branch(es):");
      state = 1;
    } else if (state == 1) {  // body
      char commit = message[message.length() - 1];
      if (commit == '1') {
        tft.setTextColor(GREEN);
      } else {
        tft.setTextColor(RED);
      }
      tft.println(" " + message.substring(0, message.length() - 1));
    }
  }

  state = 0;
  
  while (Serial.available() <= 0) {
    delay(500);
  }
}
