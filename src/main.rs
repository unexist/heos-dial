///
/// @package heos-dial
///
/// @file Main file
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

mod wifi;
mod encoder;

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

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

#[cfg(any(esp32, esp32s2, esp32s3))]
fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use encoder::Encoder;
    use wifi::Wifi;

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

    let _wifi = match Wifi(
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
