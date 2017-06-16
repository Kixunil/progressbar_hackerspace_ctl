
/// Error type returned when HTTP request fails.
#[derive(Debug)]
pub enum ResponseError {
    /// The request failed.
    Request(::reqwest::Error),
    /// The server returned error status.
    Status(::reqwest::StatusCode),
}

impl ResponseError {
    /// Convenience conversion function.
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

