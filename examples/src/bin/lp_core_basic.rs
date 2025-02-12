//! This shows a very basic example of running code on the LP core.
//!
//! Code on LP core increments a counter and continuously toggles GPIO1. The
//! current value is printed by the HP core.
//!
//! Make sure to first compile the `esp-lp-hal/examples/blinky.rs` example

//% CHIPS: esp32c6

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::lp_gpio::IntoLowPowerPin,
    lp_core::{LpCore, LpCoreWakeupSource},
    peripherals::Peripherals,
    prelude::*,
    IO,
};
use esp_println::{print, println};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    // configure GPIO 1 as LP output pin
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let lp_pin = io.pins.gpio1.into_low_power().into_push_pull_output();

    let mut lp_core = LpCore::new(peripherals.LP_CORE);
    lp_core.stop();
    println!("lp core stopped");

    // load code to LP core
    let lp_core_code =
        load_lp_code!("../esp-lp-hal/target/riscv32imac-unknown-none-elf/release/examples/blinky");

    // start LP core
    lp_core_code.run(&mut lp_core, LpCoreWakeupSource::HpCpu, lp_pin);
    println!("lpcore run");

    let data = (0x5000_2000) as *mut u32;
    loop {
        print!("Current {:x}           \u{000d}", unsafe {
            data.read_volatile()
        });
    }
}
