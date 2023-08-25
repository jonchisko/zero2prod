use reqwest::Client;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    base_url: String,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, base_url: String) -> EmailClient {
        EmailClient {
            sender,
            http_client: Client::new(),
            base_url,
        }
    }

    pub async fn send_email(
        &self,
        recipent: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
