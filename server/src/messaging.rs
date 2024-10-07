use common::APP_CONFIG;
use percent_encoding::NON_ALPHANUMERIC;

pub async fn send_message(phone_number: &str, message: &str, from: &str) -> minreq::Response {
    let body = AtMessage::new()
        .use_sandox()
        .add_from("BadiliX")
        .add_recipient(phone_number)
        .add_from(from)
        .add_message(message)
        .build();
    smol::unblock(|| {
        minreq::post("https://api.sandbox.africastalking.com/version1/messaging")
            .with_header("Accept", "application/json")
            .with_header("Content-Type", "application/x-www-form-urlencoded")
            .with_header("apiKey", APP_CONFIG.api_key())
            .with_body(body)
            .send()
            .unwrap()
    })
    .await
}

#[derive(Debug, Default)]
pub struct AtMessage {
    pub username: String,
    pub to: Vec<String>,
    pub message: String,
    pub from: String,
}

impl AtMessage {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn add_username(mut self, username: &str) -> Self {
    //     self.username = username.to_string();

    //     self
    // }

    pub fn use_sandox(mut self) -> Self {
        self.username = "sandbox".to_string();

        self
    }

    pub fn add_recipient(mut self, recipient: &str) -> Self {
        self.to.push(self.utf8_encode(recipient));

        self
    }

    pub fn add_message(mut self, message: &str) -> Self {
        self.message = self.utf8_encode(message);

        self
    }

    pub fn add_from(mut self, from: &str) -> Self {
        self.from = from.to_string();

        self
    }

    pub fn build(self) -> String {
        let mut outcome = String::new();
        outcome.push_str(&format!("username={0}&to=", self.username));

        self.to.iter().for_each(|recipient| {
            outcome.push_str(recipient);
            outcome.push(',')
        });

        outcome.push_str(&format!("&message={0}&from={1}", self.message, self.from));

        dbg!(&outcome);

        outcome
    }

    fn utf8_encode(&self, value: &str) -> String {
        percent_encoding::utf8_percent_encode(&value, NON_ALPHANUMERIC).to_string()
    }
}
