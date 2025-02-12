//! Demonstrates generating pulse sequences with RMT
//!
//! Connect a logic analyzer to GPIO4 to see the generated pulses.

//% CHIPS: esp32 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    rmt::{PulseCode, TxChannel, TxChannelConfig, TxChannelCreator},
    Delay,
    Rmt,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32h2")] {
            let freq = 32u32.MHz();
        } else {
            let freq = 80u32.MHz();
        }
    };

    let rmt = Rmt::new(peripherals.RMT, freq, &clocks).unwrap();

    let tx_config = TxChannelConfig {
        clk_divider: 255,
        ..TxChannelConfig::default()
    };

    let mut channel = rmt.channel0.configure(io.pins.gpio4, tx_config).unwrap();

    let mut delay = Delay::new(&clocks);

    let mut data = [PulseCode {
        level1: true,
        length1: 200,
        level2: false,
        length2: 50,
    }; 20];

    data[data.len() - 2] = PulseCode {
        level1: true,
        length1: 3000,
        level2: false,
        length2: 500,
    };
    data[data.len() - 1] = PulseCode::default();

    loop {
        let transaction = channel.transmit(&data);
        channel = transaction.wait().unwrap();
        delay.delay_ms(500u32);
    }
}
