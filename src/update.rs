pub fn update(url: &str, basic_auth: Option<(&str, &str)>) {
    let mut request = ureq::get(url);

    if let Some(user, password) = basic_auth {
        request = request.auth(user, password)
    }

    let _response = request.call();
}
