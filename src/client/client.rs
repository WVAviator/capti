use lazy_static::lazy_static;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    static ref CLIENT: reqwest::Client = {
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Unable to establish request client.")
    };
}

pub fn get_client() -> &'static reqwest::Client {
    &CLIENT
}
