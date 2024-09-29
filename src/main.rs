///
/// @package geos-dial
///
/// @file Main file
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

mod wifi;

use anyhow::{bail, Result};
use core::str;
use embedded_svc::{
    io::Read,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};
use esp_idf_svc::hal::{
    delay::FreeRtos,
    peripherals::Peripherals,
    prelude::*,
};
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("serc")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

#[cfg(any(esp32, esp32s2, esp32s3))]
fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use encoder::Encoder;

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("Setup pins");
    let peripherals = Peripherals::take().context("Failed to take Peripherals")?;
    let mut pin_a = peripherals.pins.gpio5;
    let mut pin_b = peripherals.pins.gpio6;

    println!("Setup encoder");
    let encoder = Encoder::new(peripherals.pcnt0, &mut pin_a, &mut pin_b)?;
    let mut last_value = 0i32;

    // Connect to the Wi-Fi network
    let app_config = CONFIG;
    let sysloop = EspSystemEventLoop::take()?;

    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            let value = encoder.get_value()?;

            if value != last_value {
                println!("value: {value}");
                last_value = value;
            }
            FreeRtos::delay_ms(100u32);
            inner
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };
    Ok(())
}

#[cfg(not(any(esp32, esp32s2, esp32s3)))]
fn main() {
    use esp_idf_svc::hal::delay::FreeRtos;
    println!("pcnt peripheral not supported on this device!");
    loop {
        FreeRtos::delay_ms(100u32);
    }
}

// esp-idf encoder implementation using v4 pcnt api
#[cfg(any(esp32, esp32s2, esp32s3))]
mod encoder {
    use std::cmp::min;
    use std::sync::atomic::AtomicI32;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use esp_idf_svc::hal::peripheral::Peripheral;
    use esp_idf_svc::hal::gpio::AnyInputPin;
    use esp_idf_svc::hal::gpio::InputPin;
    use esp_idf_svc::hal::pcnt::*;
    use esp_idf_svc::sys::EspError;

    const LOW_LIMIT: i16 = -100;
    const HIGH_LIMIT: i16 = 100;

    pub struct Encoder<'d> {
        unit: PcntDriver<'d>,
        approx_value: Arc<AtomicI32>,
    }

    impl<'d> Encoder<'d> {
        pub fn new<PCNT: Pcnt>(
            pcnt: impl Peripheral<P = PCNT> + 'd,
            pin_a: impl Peripheral<P = impl InputPin> + 'd,
            pin_b: impl Peripheral<P = impl InputPin> + 'd,
        ) -> Result<Self, EspError> {
            let mut unit = PcntDriver::new(
                pcnt,
                Some(pin_a),
                Some(pin_b),
                Option::<AnyInputPin>::None,
                Option::<AnyInputPin>::None,
            )?;
            unit.channel_config(
                PcntChannel::Channel0,
                PinIndex::Pin0,
                PinIndex::Pin1,
                &PcntChannelConfig {
                    lctrl_mode: PcntControlMode::Reverse,
                    hctrl_mode: PcntControlMode::Keep,
                    pos_mode: PcntCountMode::Decrement,
                    neg_mode: PcntCountMode::Increment,
                    counter_h_lim: HIGH_LIMIT,
                    counter_l_lim: LOW_LIMIT,
                },
            )?;
            unit.channel_config(
                PcntChannel::Channel1,
                PinIndex::Pin1,
                PinIndex::Pin0,
                &PcntChannelConfig {
                    lctrl_mode: PcntControlMode::Reverse,
                    hctrl_mode: PcntControlMode::Keep,
                    pos_mode: PcntCountMode::Increment,
                    neg_mode: PcntCountMode::Decrement,
                    counter_h_lim: HIGH_LIMIT,
                    counter_l_lim: LOW_LIMIT,
                },
            )?;

            unit.set_filter_value(min(10 * 80, 1023))?;
            unit.filter_enable()?;

            let approx_value = Arc::new(AtomicI32::new(0));
            // unsafe interrupt code to catch the upper and lower limits from the encoder
            // and track the overflow in `value: Arc<AtomicI32>` - I plan to use this for
            // a wheeled robot's odomerty
            unsafe {
                let approx_value = approx_value.clone();
                unit.subscribe(move |status| {
                    let status = PcntEventType::from_repr_truncated(status);
                    if status.contains(PcntEvent::HighLimit) {
                        approx_value.fetch_add(HIGH_LIMIT as i32, Ordering::SeqCst);
                    }
                    if status.contains(PcntEvent::LowLimit) {
                        approx_value.fetch_add(LOW_LIMIT as i32, Ordering::SeqCst);
                    }
                })?;
            }
            unit.event_enable(PcntEvent::HighLimit)?;
            unit.event_enable(PcntEvent::LowLimit)?;
            unit.counter_pause()?;
            unit.counter_clear()?;
            unit.counter_resume()?;

            Ok(Self { unit, approx_value })
        }

        pub fn get_value(&self) -> Result<i32, EspError> {
            let value =
                self.approx_value.load(Ordering::Relaxed) + self.unit.get_counter_value()? as i32;
            Ok(value)
        }
    }
}
