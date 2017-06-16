use ::reqwest::Client;
use ::error::ResponseError;

pub struct Door {
    client: Client,
}

impl Door {
    pub fn new() -> ::reqwest::Result<Self> {
        Client::new().map(|client| Door { client })
    }

    pub fn open_outside(&mut self) -> Result<(), ResponseError> {
        ResponseError::from_response(
            self.client
                .get("http://192.168.223.11/outsidedoor")
                .send()
        )
        .map(::std::mem::drop)
    }

    pub fn enable_auto(&mut self) -> Result<(), ResponseError> {
        ResponseError::from_response(
            self.client
                .get("http://192.168.223.11/enableauto")
                .send()
        )
        .map(::std::mem::drop)
    }

    pub fn disable_auto(&mut self) -> Result<(), ResponseError> {
        ResponseError::from_response(
            self.client
                .get("http://192.168.223.11/disableauto")
                .send()
        )
        .map(::std::mem::drop)
    }
}
