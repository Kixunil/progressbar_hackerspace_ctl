extern crate progressbar_hackerspace_ctl;

use progressbar_hackerspace_ctl::led_strip::{LedStrip, LedStripColor};

fn main() {
    let mut led_strip = LedStrip::new().unwrap();
    led_strip.set_color(LedStripColor::from_rgb8(0xFF, 0x00, 0x00)).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    led_strip.set_color(LedStripColor::from_rgb8(0x00, 0xFF, 0x00)).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    led_strip.set_color(LedStripColor::from_rgb8(0x00, 0x00, 0xFF)).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    led_strip.set_color(LedStripColor::from_rgb8(0xFF, 0xFF, 0xFF)).unwrap();
}
