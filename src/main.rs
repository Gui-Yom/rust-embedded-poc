#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use atsam3x8e::Peripherals;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    //peripherals.PIOB.absr.write(|w| w.p27().set_bit());
    peripherals.PIOB.per.write(|w| w.p27().set_bit());
    peripherals.PIOB.oer.write(|w| w.p27().set_bit());
    peripherals.PIOB.ower.write(|w| w.p27().set_bit());
    loop {
        peripherals.PIOB.sodr.write(|w| w.p27().set_bit());
        peripherals.PIOB.odsr.write(|w| w.p27().set_bit());
        for _ in 0..100000 {
            cortex_m::asm::nop();
        }
        peripherals.PIOB.codr.write(|w| w.p27().set_bit());
        for _ in 0..100000 {
            cortex_m::asm::nop();
        }
    }
}
