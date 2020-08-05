#![no_std]
#![no_main]

#[allow(unused_extern_crates)]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

use stm32l4::stm32l4x2::{interrupt, Interrupt, NVIC, gpioa};



#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;

    hprintln!("Hello, world!").unwrap();

    unsafe { NVIC::unmask(Interrupt::EXTI0); }

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();

    loop {
        while !syst.has_wrapped() {}

        // trigger the `EXTI0` interrupt
        NVIC::pend(Interrupt::EXTI0);
    }
}

#[interrupt]
fn EXTI0() {
    hprint!(".").unwrap();
}
