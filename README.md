# LCD Display Driver for Rust

[![Crates.io](https://img.shields.io/crates/v/lcd_display.svg)](https://crates.io/crates/lcd_display)
[![Documentation](https://docs.rs/lcd_display/badge.svg)](https://docs.rs/lcd_display)
[![License](https://img.shields.io/crates/l/lcd_display.svg)](LICENSE)

A Rust library for controlling character LCD displays (HD44780 compatible) via GPIO pins on Linux systems. This crate provides a simple and safe interface for displaying text on LCD screens commonly used in embedded projects.

## Features

- **4-bit and 8-bit mode support**: Flexible pin configuration
- **Thread-safe GPIO control**: Built on top of `gpio-cdev`
- **Simple API**: Easy to use with clear method names
- **Linux compatibility**: Works with any Linux system that exposes GPIO via `/dev/gpiochip*`
- **HD44780 compatible**: Works with most character LCD displays

## Hardware Requirements

- Linux system with GPIO support (Raspberry Pi, BeagleBone, etc.)
- HD44780 compatible character LCD display
- Appropriate wiring between GPIO pins and LCD

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lcd_display = "0.1.0"
```

## Quick Start

Here's a simple example showing how to use the library:

```rust
use lcd_display::{GPIO_Pin, LCD, LCD_Mode};

fn main() {
	// Control pins
	let rs = GPIO_Pin::new("/dev/gpiochip1", 03).expect("Error while opening RS pin!");
	let en = GPIO_Pin::new("/dev/gpiochip1", 04).expect("Error while opening EN pin!");

	// Data pins
	let d4 = GPIO_Pin::new("/dev/gpiochip0", 12).expect("Error while opening D4 pin!");
	let d5 = GPIO_Pin::new("/dev/gpiochip3", 26).expect("Error while opening D5 pin!");
	let d6 = GPIO_Pin::new("/dev/gpiochip0", 14).expect("Error while opening D6 pin!");
	let d7 = GPIO_Pin::new("/dev/gpiochip1", 01).expect("Error while opening D7 pin!");


    let data_pins = vec![
        d4,
        d5,
        d6,
        d7
    ];

	let mut lcd = LCD::new(
        rs,
        en,
        data_pins,
        LCD_Mode::FourBit
    );
    
    lcd.begin(20, 4);

	lcd.set_cursor(0, 0);
    lcd.print("Hello!");

    lcd.set_cursor(0, 1);
    lcd.print("LCD");

    lcd.set_cursor(0, 2);
    lcd.print("with");

    lcd.set_cursor(0, 3);
    lcd.print("Rust!");

    std::thread::sleep(std::time::Duration::from_secs(30));

    lcd.clear();
}
```

## API Documentation

### LCD Modes

The library supports two modes:

- `LCD_Mode::FourBit`: Uses 4 data pins (D4-D7)
- `LCD_Mode::EightBit`: Uses 8 data pins (D0-D7)

### Main Methods

#### `LCD::new(rs, en, data_pins, mode)`
Creates a new LCD instance.
- `rs`: Register Select pin
- `en`: Enable pin  
- `data_pins`: Vector of data pins (4 for FourBit mode, 8 for EightBit mode)
- `mode`: LCD_Mode enum value

#### `lcd.begin(columns, rows)`
Initializes the LCD display with the specified dimensions.

#### `lcd.print(text)`
Displays text at the current cursor position.

#### `lcd.set_cursor(column, row)`
Sets the cursor position (0-indexed).

#### `lcd.clear()`
Clears the display and returns cursor to home position.

#### `lcd.get_columns()` / `lcd.get_rows()`
Returns the configured display dimensions.

### GPIO Pin Management

The `GPIO_Pin` struct handles individual GPIO pin control:

#### `GPIO_Pin::new(chip_path, line_offset)`
Creates a new GPIO pin instance.
- `chip_path`: Path to GPIO chip (e.g., "/dev/gpiochip0")
- `line_offset`: Pin number on the chip

## Wiring Guide

### 4-bit Mode (Recommended)
```
LCD Pin | Description       | GPIO Pin
--------|-------------------|----------
VSS     | Ground            | GND
VDD     | Power (+5 V)      | +5 V
V0      | Contrast          | Potentiometer center
RS      | Register Select   | Your choice
RW      | Read/Write Select | GND
Enable  | Enable            | Your choice
D4      | Data 4            | Your choice
D5      | Data 5            | Your choice
D6      | Data 6            | Your choice
D7      | Data 7            | Your choice
A       | Backlight +       | +5 V
K       | Backlight -       | GND
```

### 8-bit Mode
Similar to 4-bit mode but also connect D0, D1, D2, D3 data pins.

## Examples

### Basic Text Display
```rust
lcd.begin(16, 2);
lcd.print("Hello World!");
```

### Multi-line Display
```rust
lcd.begin(20, 4);

lcd.set_cursor(0, 0);
lcd.print("Line 1");

lcd.set_cursor(0, 1);
lcd.print("Line 2");

lcd.set_cursor(0, 2);
lcd.print("Line 3");

lcd.set_cursor(0, 3);
lcd.print("Line 4");
```

### Clearing and Updating
```rust
lcd.clear();
lcd.set_cursor(1, 1);
lcd.print("Centered Text!");
```

## Error Handling

The library uses `Result` types for error handling. Most methods that can fail return `Result<(), Box<dyn std::error::Error>>`.

Common errors:
- GPIO chip not found
- Permission denied (run with appropriate privileges)
- Invalid pin numbers
- Hardware connection issues

## Performance Notes

- The library includes appropriate delays for LCD timing requirements
- 4-bit mode is generally recommended for most applications
- Operations are blocking and not suitable for real-time applications

## Contributing

Contributions are welcome! Please feel free to:

1. Open issues for bugs or feature requests
2. Submit pull requests
3. Improve documentation
4. Add examples

## License

This project is licensed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of the excellent [`gpio-cdev`](https://crates.io/crates/gpio-cdev) crate
- Inspired by the Arduino LiquidCrystal library
- HD44780 datasheet and community documentation

## Changelog

### v0.1.0
- Initial release
- 4-bit and 8-bit mode support
- Basic LCD operations (print, clear, cursor control)
- Linux GPIO support via gpio-cdev
