use crate::utils::config::Config;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(config: Config, subject: String, body: String) {
    let message = Message::builder()
        .from(config.smtp_from.parse().unwrap())
        .to(config.smtp_to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let mailer = SmtpTransport::starttls_relay(&config.smtp_host)
        .unwrap()
        .port(config.smtp_port)
        .build();

    mailer.send(&message).unwrap();
}