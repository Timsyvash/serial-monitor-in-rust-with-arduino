#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::*;

use ufmt::uwriteln;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);

    loop {
        uwriteln!(&mut serial, "Hello Embedded Rust!\r\n").unwrap();
        delay_ms(1000);
    }
}
