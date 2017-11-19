#![feature(asm, lang_items, unwind_attributes)]

#![no_std]
#![no_main]

extern crate arduino;
extern crate avr_delay;

use arduino::{DDRB, PORTB, PORTB5};
use core::ptr::{write_volatile, read_volatile};
use avr_delay::delay_ms;

// Blinking delay in ms
const LED_BLINK_DELAY: u32 = 1000;

#[no_mangle]
pub extern fn main() {
    // Set all PORTB pins up as outputs
    unsafe { write_volatile(DDRB, 0xFF) }

    loop {
        // Set the builtin LED pin to high
        unsafe { write_volatile(PORTB, read_volatile(PORTB) ^ PORTB5) }

        delay_ms(LED_BLINK_DELAY);
    }
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        loop { }
    }
}
