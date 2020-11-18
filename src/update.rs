use std::error::Error;
use std::fmt::{Debug, Display};

pub fn update(url: &str, basic_auth: Option<(&str, &str)>) -> Result<String, HttpError> {
    let mut request = if let Some((user, password)) = basic_auth {
        ureq::get(url).auth(user, password).clone()
    } else {
        ureq::get(url).clone()
    };

    let response = request.call();
    return if let Some(error) = response.synthetic_error() {
        Err(HttpError { status_code: error.status() })
    } else {
        Ok(response.into_string().unwrap_or(String::new()))
    }

}

#[derive(Debug)]
pub struct HttpError {
    status_code: u16,
}

impl Display for HttpError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "HTTP error: {}", self.status_code)
    }
}

impl Error for HttpError {}
