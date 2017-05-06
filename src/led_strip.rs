use ::reqwest::Client;

/// Type used for LED strip colors as understood by remote device
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LedStripColor {
    r: u16,
    g: u16,
    b: u16,
}

impl LedStripColor {
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

    pub fn from_raw(r: u16, g: u16, b: u16) -> Option<Self> {
        if r < 1024 && g < 1024 && b < 1024 {
            Some(LedStripColor { r, g, b })
        } else {
            None
        }
    }

    pub fn from_raw_trim(r: u16, g: u16, b: u16) -> Self {
        use std::cmp::min;

        LedStripColor {
            r: min(r, 1023),
            g: min(g, 1023),
            b: min(b, 1023),
        }
    }

    pub fn into_raw(self) -> (u16, u16, u16) {
        (self.r, self.g, self.b)
    }
}

#[derive(Debug)]
pub enum ResponseError {
    Request(::reqwest::Error),
    Status(::reqwest::StatusCode),
}

impl ResponseError {
    pub fn from_response(response: ::reqwest::Result<::reqwest::Response>) -> Result<::reqwest::Response, Self> {
        response.map_err(ResponseError::Request)
            .and_then(|response|
                match *response.status() == ::reqwest::StatusCode::Ok {
                    true => Ok(response),
                    false => Err(ResponseError::Status(*response.status())),
                }
            )
    }
}

pub struct LedStrip {
    client: Client,
}

impl LedStrip {
    pub fn new() -> ::reqwest::Result<Self> {
        Client::new().map(|client| LedStrip { client })
    }

    pub fn set_color(&mut self, color: LedStripColor) -> Result<(), ResponseError> {
        let (r, g, b) = color.into_raw();
        ResponseError::from_response(
            self.client
                .get(&format!("http://192.168.223.59/?r={}&g={}&b={}", r, g, b))
                .send()
        )
        .map(::std::mem::drop)
    }
}
