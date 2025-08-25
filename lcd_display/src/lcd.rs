/*
	lcd.rs file
*/

use crate::gpio_pin::GPIO_Pin;

use std::{
	thread,
	time::Duration
};

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum LCD_Mode {
    FourBit,
    EightBit
}

#[derive(Debug)]
pub struct LCD {
    rs: GPIO_Pin,
    en: GPIO_Pin,
    data_pins: Vec<GPIO_Pin>,
    mode: LCD_Mode,
    columns: u8,
    rows: u8
}

impl LCD {
    pub fn new(
        rs: GPIO_Pin,
        en: GPIO_Pin,
        data_pins: Vec<GPIO_Pin>,
        mode: LCD_Mode
    ) -> Self {
        match mode {
			LCD_Mode::FourBit => assert_eq!(data_pins.len(), 4, "A four bit mode requires exactly 4 data pins!"),
            LCD_Mode::EightBit => assert_eq!(data_pins.len(), 8, "A eight bit mode requires exactly 8 data pins!")
		}

        LCD {
            rs,
            en,
            data_pins,
            mode,
            columns: 0,
            rows: 0
        }
    }

    pub fn begin(&mut self, columns: u8, rows: u8) {
		self.columns = columns;
        self.rows = rows;

        thread::sleep(Duration::from_millis(50));

		self.rs.set_low().unwrap();
        self.en.set_low().unwrap();

        match self.mode {
            LCD_Mode::FourBit => {
				self.write_nibble(0x03);
				thread::sleep(Duration::from_millis(5));

				self.write_nibble(0x03);
				thread::sleep(Duration::from_millis(5));

				self.write_nibble(0x03);
				thread::sleep(Duration::from_micros(150));

				self.write_nibble(0x02);
				thread::sleep(Duration::from_micros(150));

				let command = if rows > 1 { 0x28 } else { 0x20 };
				self.command(command);
			}    
			LCD_Mode::EightBit => {
				self.write_bits(0x30);
				thread::sleep(Duration::from_millis(5));

				self.write_bits(0x30);
				thread::sleep(Duration::from_micros(150));

				self.write_bits(0x30);
				thread::sleep(Duration::from_micros(150));

				let command = if rows > 1 { 0x38 } else { 0x30 };
				self.command(command);                
            }
        }

        thread::sleep(Duration::from_millis(1));

        self.command(0x0C);
        thread::sleep(Duration::from_micros(50));

        self.clear();

        self.command(0x06);
        thread::sleep(Duration::from_micros(50));

        thread::sleep(Duration::from_millis(100));
    }

    pub fn clear(&mut self) {
        self.command(0x01);
        thread::sleep(Duration::from_millis(2));
    }

    pub fn set_cursor(&mut self, column: u8, row: u8) {
        let row_offsets = [
            0x00,
            0x40,
            0x14,
            0x54
        ];

        if row >= self.rows {
            panic!("Invalid row: {} | Max row: {}", row, self.rows - 1);
        }

        let address = column + row_offsets[row as usize];
        self.command(0x80 | address);
    }

    pub fn print(&mut self, text: &str) {
        for character in text.chars() {
            self.write_char(character as u8);
            thread::sleep(Duration::from_micros(150));
        }

        thread::sleep(Duration::from_micros(150));
    }

	pub fn get_columns(self) -> u8 {
		self.columns
	}

	pub fn get_rows(self) -> u8 {
		self.rows
	}

    fn pulse_enable(&mut self) {
        self.en.set_low().unwrap();
        thread::sleep(Duration::from_micros(2));

        self.en.set_high().unwrap();
        thread::sleep(Duration::from_micros(2));

        self.en.set_low().unwrap();
        thread::sleep(Duration::from_millis(1));
    }

    fn write_nibble(&mut self, value: u8) {
        for index in 0..4 {
            let bit = (value >> index) & 0x01;

            if bit != 0 {
                self.data_pins[index].set_high().unwrap();
            }
            else {
                self.data_pins[index].set_low().unwrap();
            }
        }

        self.pulse_enable();
    }

    fn write_bits(&mut self, value: u8) {
        for index in 0..self.data_pins.len() {
            let bit = (value >> index) & 0x01;

            if bit != 0 {
                self.data_pins[index].set_high().unwrap();
            }
            else {
                self.data_pins[index].set_low().unwrap();
            }
        }

        self.pulse_enable();
    }

    fn send(&mut self, value: u8, is_data: bool) {
        if is_data {
            self.rs.set_high().unwrap();
        }
        else {
            self.rs.set_low().unwrap();
        }

        thread::sleep(Duration::from_micros(100));

        if self.mode == LCD_Mode::FourBit {
			self.write_nibble(value >> 4);
			self.write_nibble(value & 0x0F);
		}
		else {
			self.write_bits(value);
		}
    }

    fn write_char(&mut self, value: u8) {
        self.send(value, true);
    }

    fn command(&mut self, value: u8) {
        self.send(value, false);
    }
}
