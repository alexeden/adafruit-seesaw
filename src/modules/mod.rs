pub mod status;

pub type Reg = [u8; 2];

pub const SEESAW_HW_ID: u8 = 0x55;
const STATUS_MODULE_ID: u8 = 0x00;
const GPIO_MODULE_ID: u8 = 0x01;
const SERCOM0_MODULE_ID: u8 = 0x02;
const TIMER_MODULE_ID: u8 = 0x08;
const ADC_MODULE_ID: u8 = 0x09;
const DAC_MODULE_ID: u8 = 0x0A;
const INTERRUPT_MODULE_ID: u8 = 0x0B;
const DAP_MODULE_ID: u8 = 0x0C;
const EEPROM_MODULE_ID: u8 = 0x0D;
const NEOPIXEL_MODULE_ID: u8 = 0x0E;
const TOUCH_MODULE_ID: u8 = 0x0F;
const KEYPAD_MODULE_ID: u8 = 0x10;
const ENCODER_MODULE_ID: u8 = 0x11;
const SPECTRUM_MODULE_ID: u8 = 0x12;
