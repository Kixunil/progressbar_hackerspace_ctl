
extern crate progressbar_hackerspace_ctl;

use progressbar_hackerspace_ctl::led_strip::{LedStrip, LedStripColor};

fn main() {
    let mut led_strip = LedStrip::new().unwrap();
    let r = LedStripColor::from_rgb8(0xFF, 0x00, 0x00);
    let g = LedStripColor::from_rgb8(0x00, 0xFF, 0x00);
    let b = LedStripColor::from_rgb8(0x00, 0x00, 0xFF);

    let iter = r.fade_to(b, 50)
        .chain(b.fade_to(g, 50))
        .chain(g.fade_to(r, 50))
        .cycle();

    for color in iter {
         led_strip.set_color(color).unwrap();
         std::thread::sleep(std::time::Duration::from_millis(10))
    }
}
