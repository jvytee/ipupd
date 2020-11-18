use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    interface: String,
    domain: String,
    url: String,
    user: Option<String>,
    password: Option<String>,
}
