#![feature(no_std)]
#![feature(core)]
#![no_std]

//! Toggle the blue LED (PC8) at 1 Hz. The timing is handled by the TIM7 timer. The main thread
//! sleeps most of the time, and only wakes up on TIM7's interrupts to toggle the LED.

//extern crate cortex;
extern crate core;
extern crate nrf51822;
extern crate cortex;

use nrf51822::interrupt::IntVector;
use cortex::nvic::iser0;

const CC0VAL: u32 = 16384;

pub fn get_nvic_iser0_mask(vector: IntVector) -> iser0::Bit {
	match vector {
		nrf51822::interrupt::IntVector::RTC0Irqn => iser0::Bit::_11
	}
}

pub fn sys_init() {
	let clock = nrf51822::peripheral::clock();

	clock.tasks_lfclkstart.set(1);
}

#[no_mangle]
pub fn main() {
    let gpio = nrf51822::peripheral::gpio();
    let rtc0 = nrf51822::peripheral::rtc0();
    let nvic = cortex::peripheral::nvic();
    
    sys_init();

    rtc0.power.set(1);
    rtc0.intenset.update(|intenset| {
    	use nrf51822::rtc::intenset::prelude::*;

    	intenset | COMPARE0
    });

    rtc0.cc0.set(CC0VAL);

    // unmask RTC0 interrupt
    nvic.iser0.set({
        //int_mask
        get_nvic_iser0_mask(IntVector::RTC0Irqn)
    });

    gpio.dir_set.update(|dir_set| {
    	use nrf51822::gpio::dir_set::prelude::*;

    	dir_set | PIN25 | PIN24
    });

    let mut state: bool = true;

    rtc0.tasks_start.set(1);
    loop {
    	if state {
	        gpio.out_set.update(|out_set| {
	        	use nrf51822::gpio::out_set::prelude::*;

	        	out_set | PIN25 | PIN24
	        });
    	} else {
    		gpio.out_clr.update(|out_clr| {
        		use nrf51822::gpio::out_clr::prelude::*;

        		out_clr | PIN25 | PIN24
        	});	
    	}

        cortex::asm::wfi();

        if state == true {
        	state = false;
        	
        	rtc0.cc0.set(CC0VAL);
        } else {
        	state = true;
        	rtc0.cc0.set(CC0VAL);
        }	

        rtc0.tasks_clear.set(1);
        rtc0.tasks_start.set(1);	        
    }
}

#[no_mangle]
pub extern fn rtc0() {
	let rtc0 = nrf51822::peripheral::rtc0();

	rtc0.events_compare0.set(0);
	rtc0.tasks_stop.set(1);
}