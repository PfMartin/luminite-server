use anyhow::Result;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

pub fn set_led_color(r: u8, g: u8, b: u8) -> Result<()> {
    let mut led_strip = Ws2812Esp32Rmt::new(0, 6)?;

    let color = RGB8::new(r, g, b);

    let pixels = std::iter::repeat(color).take(61);
    led_strip.write(pixels)?;

    Ok(())
}
