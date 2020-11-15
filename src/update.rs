pub fn update(url: &str, basic_auth: Option<(&str, &str)>) -> Result<String>{
    let mut request = ureq::get(url);

    if let Some(user, password) = basic_auth {
        request = request.auth(user, password)
    }

    let response = request.call();
    return if let Some(error) = response.synthetic_error() {
        Err(error)
    } else {
        Ok(response.into_string().unwrap_or(String::new()))
    }
}
