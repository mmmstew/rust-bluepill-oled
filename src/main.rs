// src/main.rs

// Flashes "Hello World!" every 100ms, for 100ms using the ubiquitous STM32F103C8-based "Bluepill" connected to the equally ubiquitous SSD1306-based 0.96" OLED available via Aliexpress etc.
// SDA -> PB9
// SCL -> PB8

// Usage:
// cargo build --release
// cargo flash --chip STM32F103C8 --connect-under-reset --release

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry; // The runtime
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    pac,
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOB peripheral and configure alternate function as open drain for I2C
    let mut gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // HAL I2C config
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        /*&mut rcc.apb1,*/
        1000,
        10,
        1000,
        1000,
    );

    // ssd1306 driver config
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        display.clear();

        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        delay.delay_ms(100_u16);

        display.clear();

        display.flush().unwrap();

        delay.delay_ms(100_u16);
    }
}
