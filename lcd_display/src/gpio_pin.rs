/*
	gpio_pin.rs file
*/

use gpio_cdev::{
	Chip,
	LineHandle,
	LineRequestFlags
};

use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct GPIO_Pin {
	handle: LineHandle
}

impl GPIO_Pin {
	pub fn new<P: AsRef<Path>>(chip_path: P, line_offset: u32) -> Result<Self, Box<dyn std::error::Error>> {
		let mut chip = Chip::new(chip_path)?;
		let handle = chip
			.get_line(line_offset)?
			.request(LineRequestFlags::OUTPUT, 0, "gpio_pin_lcd")?;
		
		Ok(GPIO_Pin { handle })
	}

	pub fn set_high(&self) -> Result<(), Box<dyn std::error::Error>> {
		self.handle.set_value(1)?;

		Ok(())
	}

	pub fn set_low(&self) -> Result<(), Box<dyn std::error::Error>> {
		self.handle.set_value(0)?;

		Ok(())
	}
}
