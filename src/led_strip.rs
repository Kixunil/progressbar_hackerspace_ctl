//! This module contains types and methods for controlling the LED strip attached to the door
//! of the presentation room.

use ::reqwest::Client;
use ::error::ResponseError;

/// Type used for LED strip colors as understood by remote device
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LedStripColor {
    r: u16,
    g: u16,
    b: u16,
}

impl LedStripColor {
    /// Creates the color from RGB triple.
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        let rgb = [r, g, b];
        let mut iter = rgb.iter()
            .cloned()
            .map(Into::into)
                // `as` won't fail here because max value of `c` is 255.
            .map(|c: u32| (1023 - c * 1023 / 255) as u16);

        let r = iter.next().unwrap();
        let g = iter.next().unwrap();
        let b = iter.next().unwrap();

        LedStripColor {
            r,
            g,
            b,
        }
    }

    /// Creates the color directly using values. All values must be less than 1024.
    /// Note that raw numbers use 0 for max intesity and 1023 for min!
    pub fn from_raw(r: u16, g: u16, b: u16) -> Option<Self> {
        if r < 1024 && g < 1024 && b < 1024 {
            Some(LedStripColor { r, g, b })
        } else {
            None
        }
    }

    /// Creates the color directly using values. All values above 1023 will be capped to 1023.
    /// Note that raw numbers use 0 for max intesity and 1023 for min!
    pub fn from_raw_trim(r: u16, g: u16, b: u16) -> Self {
        use std::cmp::min;

        LedStripColor {
            r: min(r, 1023),
            g: min(g, 1023),
            b: min(b, 1023),
        }
    }

    /// Gets the raw values. Returned tuple is ordered as R, G, B.
    pub fn into_raw(self) -> (u16, u16, u16) {
        (self.r, self.g, self.b)
    }

    /// Creates iterator returning colors, fading from self to target.
    /// Number of steps is specified by `steps` argument. If zero is supplied, the iterator will be
    /// empty!
    pub fn fade_to(self, target: LedStripColor, steps: u16) -> Fader {
        Fader {
            state: self,
            target,
            steps,
        }
    }
}

/// Iterator over colors with fading effect. Created by calling `fade_to()` method on
/// `LedStripColor`.
#[derive(Clone, Eq, PartialEq)]
pub struct Fader {
    state: LedStripColor,
    target: LedStripColor,
    steps: u16,
}

impl Iterator for Fader {
    type Item = LedStripColor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            None
        } else {
            let (r1, g1, b1) = self.state.into_raw();
            let (r2, g2, b2) = self.target.into_raw();

            let (r1, g1, b1) = (r1 as i32, g1 as i32, b1 as i32);
            let (r2, g2, b2) = (r2 as i32, g2 as i32, b2 as i32);
            let steps = self.steps as i32;

            let sr = (r2 - r1) / steps;
            let sg = (g2 - g1) / steps;
            let sb = (b2 - b1) / steps;

            let nr = r1 + sr;
            let ng = g1 + sg;
            let nb = b1 + sb;

            let (nr, ng, nb) = (nr as u16, ng as u16, nb as u16);

            self.state = LedStripColor::from_raw_trim(nr, ng, nb);
            self.steps -= 1;
            Some(self.state)
        }
    }
}

/// Represents LED strip attached to the door in Progressbar hackerspace.
pub struct LedStrip {
    client: Client,
}

impl LedStrip {
    /// Creates an instance of LedStrip.
    pub fn new() -> ::reqwest::Result<Self> {
        Client::new().map(|client| LedStrip { client })
    }

    /// Changes color of the strip.
    pub fn set_color(&mut self, color: LedStripColor) -> Result<(), ResponseError> {
        let (r, g, b) = color.into_raw();
        ResponseError::from_response(
            self.client
                .get(&format!("http://192.168.223.8/?r={}&g={}&b={}", r, g, b))
                .send()
        )
        .map(::std::mem::drop)
    }
}
