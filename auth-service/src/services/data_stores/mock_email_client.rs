use color_eyre::eyre::Result;

use crate::domain::{email::Email, email_client::EmailClient};

pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        // println!(
        //     "Sending email to {} with subject: {} and content: {}",
        //     recipient.as_ref(),
        //     subject,
        //     content
        // );
        tracing::debug!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref(),
            subject,
            content
        );

        Ok(())
    }
}