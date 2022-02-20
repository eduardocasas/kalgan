//! A service for email sending through smtp based on [lettre crate v0.10.0-rc.4](https://docs.rs/lettre/0.10.0-rc.4/lettre/).

use crate::settings;
use lettre::transport::smtp::Error;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use log::{debug, error, info};

/// Sends an email through smtp.
/// # Examples
/// ```yaml
/// ## settings.yaml
///
/// mailer:
///   sender: John Doe <john.doe@foo.bar>
///   server: smtp.foo.bar
///   user: john.doe@foo.bar
///   password: my_password
/// ```
/// ```
/// use kalgan::service::mailer;
///
/// async fn send_email() {
///     match mailer::send_email(
///         "email_of_the_addressee@foo.bar",
///         "Subject of the Email",
///         "Body of the Email"
///     ).await {
///         Ok(()) => { println!("Email sent successfully.") },
///         Err(e) => { println!("{}", e) }
///     }
/// }
/// ```
pub async fn send_email(addressee: &str, subject: &str, body: &str) -> Result<(), Error> {
    send(
        &settings::get_string("mailer.sender").unwrap(),
        &settings::get_string("mailer.user").unwrap(),
        &settings::get_string("mailer.password").unwrap(),
        &settings::get_string("mailer.server").unwrap(),
        addressee,
        subject,
        body,
    )
    .await
}
async fn send(
    sender: &str,
    user: &str,
    password: &str,
    server: &str,
    addressee: &str,
    subject: &str,
    body: &str,
) -> Result<(), Error> {
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(addressee.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(server)
            .unwrap()
            .credentials(Credentials::new(user.to_string(), password.to_string()))
            .build();
    match mailer.send(email).await {
        Ok(response) => {
            info!("Email sent successfully!");
            debug!("{:#?}", response);
            Ok(())
        }
        Err(e) => {
            error!("Could not send email: {:?}", e);
            Err(e)
        }
    }
}
