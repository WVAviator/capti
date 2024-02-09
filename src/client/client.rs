use std::ops::Deref;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct Client(reqwest::Client);

impl Default for Client {
    fn default() -> Self {
        Client(
            reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .cookie_store(true)
                .build()
                .unwrap(),
        )
    }
}

impl Deref for Client {
    type Target = reqwest::Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Client {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
