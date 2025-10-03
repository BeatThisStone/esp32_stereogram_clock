#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    spi::master::Spi,
    time::{Duration, Instant},
    Config,
};
use mipidsi::{interface::SpiInterface, models::ST7796, Builder};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0
    let config: Config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    let set_button_pin = peripherals.GPIO5;
    let up_button_pin = peripherals.GPIO18;
    let down_button_pin = peripherals.GPIO19;
    let snooze_button_pin = peripherals.GPIO23;

    let buzzer_pin = peripherals.GPIO22;

    let display_cs_pin = peripherals.GPIO33;
    let display_rst_pin = peripherals.GPIO13;
    let display_dc_pin = peripherals.GPIO25;
    let display_mosi_pin = peripherals.GPIO26;
    let display_sck_pin = peripherals.GPIO27;
    let display_led_pin = peripherals.GPIO12;
    let display_miso_pin = peripherals.GPIO14;

    let ldr_pin = peripherals.GPIO21;

    //let rtc_sda_pin = peripherals.GPIO21;
    //let rtc_scl_pin = peripherals.GPIO20;
    //let rtc_sqw_pin = peripherals.GPIO34;

    //let dht_pin = peripherals.GPIO2;

    let mut set_button = Input::new(set_button_pin, InputConfig::default().with_pull(Pull::Up));
    let mut up_button = Input::new(up_button_pin, InputConfig::default().with_pull(Pull::Up));
    let mut down_button = Input::new(down_button_pin, InputConfig::default().with_pull(Pull::Up));
    let mut snooze_button = Input::new(
        snooze_button_pin,
        InputConfig::default().with_pull(Pull::Up),
    );

    let mut buzzer = Output::new(buzzer_pin, Level::Low, OutputConfig::default());

    let mut ldr = Input::new(ldr_pin, InputConfig::default());

    let display_spi = Spi::new(peripherals.SPI2, esp_hal::spi::master::Config::default());
    let display_spi = match display_spi {
        Err(_) => loop {},
        Ok(spi) => spi
            .with_mosi(display_mosi_pin)
            .with_sck(display_sck_pin)
            .with_miso(display_miso_pin),
    };

    let mut display_buffer = [0u8; 4096];
    let di = SpiInterface::new(display_spi, display_dc_pin, &mut display_buffer);

    let mut tft_display = Builder::new(ST7796, di).reset_pin(display_rst_pin);

    loop {
        if up_button.is_low() {
            buzzer.set_high();
        } else {
            buzzer.set_low();
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
