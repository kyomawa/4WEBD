use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};
use serde::{Deserialize, Serialize};
use std::env;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub async fn send_mail(email: SendMail) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mail_hostname = env::var("MAIL_HOSTNAME")
        .expect("❌ No env variable MAIL_HOSTNAME was found in the .env file");
    let mail_username = env::var("MAIL_USERNAME")
        .expect("❌ No env variable MAIL_USERNAME was found in the .env file");
    let mail_password = env::var("MAIL_PASSWORD")
        .expect("❌ No env variable MAIL_PASSWORD was found in the .env file");

    let from_email: Mailbox = mail_username
        .parse()
        .expect("❌ Invalid MAIL_USERNAME email address");

    let email = Message::builder()
        .from(from_email.clone())
        .reply_to(from_email.clone())
        .to(email.to.parse().unwrap())
        .subject(email.subject)
        .body(email.body)
        .expect("Failed to build email message");

    let creds = Credentials::new(mail_username, mail_password);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&mail_hostname)?
        .credentials(creds)
        .build();

    mailer.send(email).await?;

    println!("Email sent successfully!");

    Ok(())
}

// =============================================================================================================================
