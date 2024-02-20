#![no_std]
#![no_main]

mod counters;

use crate::counters::{Count, LimitCounter, Reset, Show};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

const LED_COUNT: usize = 8;
pub const COUNTER_LIMIT: usize = 60;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("LED Blinking with Limit Counter and reset button!");
    let (mut leds, button) = init_leds_and_buttons();
    info!("Create Limit Counter!");
    let limit_counter = LimitCounter::new( COUNTER_LIMIT + 6, COUNTER_LIMIT);
    match limit_counter {
        None => println!("input parameter overflow value is equal or bigger then the limit"),
        Some(mut lc) => {
            info!("Start blinking loop!");
            loop {
                blinking_loop(&mut leds, &mut lc, &button).await;
            }
        }
    }
}

pub fn init_leds_and_buttons() -> ([Output<'static>; LED_COUNT], Input<'static>) {
    let p = embassy_stm32::init(Default::default());
    info!("Get the button PC13 from the peripherals!");
    let button = Input::new(p.PC13, Pull::Down);
    info!("Get the LEDs PB8..PB15 from the peripherals!");
    let mut leds = [
        Output::new(p.PB8, Level::High, Speed::Low),
        Output::new(p.PB9, Level::High, Speed::Low),
        Output::new(p.PB10, Level::High, Speed::Low),
        Output::new(p.PB11, Level::High, Speed::Low),
        Output::new(p.PB12, Level::High, Speed::Low),
        Output::new(p.PB13, Level::High, Speed::Low),
        Output::new(p.PB14, Level::High, Speed::Low),
        Output::new(p.PB15, Level::High, Speed::Low),
    ];
    // alle ausschalten
    info!("Switch all LEDs off!");
    leds.iter_mut().for_each(|led| led.set_low());
    // leds Array zurückgeben
    info!("Return a tuple of LEDs and a button!");
    (leds, button)
}

async fn blinking_loop<T>(leds: &mut [Output<'_>; LED_COUNT], limit_counter: &mut T, button: &Input<'_>)
where
    T: Count + Reset + Show,
{
    if button.is_low() {
        leds.iter_mut().for_each(|led| led.set_high());
        limit_counter.reset();
    } else {
        leds.iter_mut().for_each(|led| led.set_low());
        blinky::blinking_loop(leds, limit_counter).await;
    }
}

mod blinky {
    use crate::counters::{Count, Show};
    use crate::LED_COUNT;
    use defmt::info;
    use embassy_stm32::gpio::Output;
    use embassy_time::Timer;

    pub const BLINK_INTERVAL: u64 = 1000;

    pub async fn blinking_loop<T: Count + Show>(leds: &mut [Output<'_>; LED_COUNT], limit_counter: &mut T) {
        let counter_value = limit_counter.get_counter();
        info!("counter value: {}", counter_value);
        let counter_remainder = counter_value % leds.len();

        for (i, led) in leds.iter_mut().enumerate() {
            if i == counter_remainder {
                blink_led(led, BLINK_INTERVAL).await;
            }
        }

        limit_counter.count();
    }

    async fn blink_led(led: &mut Output<'_>, duration_ms: u64) {
        led.set_high();
        Timer::after_millis(duration_ms).await;
        led.set_low();
    }
}
