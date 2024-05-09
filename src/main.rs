use base64::{engine::general_purpose, Engine as _};
use cli::CliOpts;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};

mod cli;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = CliOpts::parse_cli();

    let from = opts.get_from_email();
    let to = opts.get_to_email();
    let subject = opts.get_subject();
    let body = opts.get_body();
    let smtp_host = opts.get_smtp_host();
    let smtp_port = opts.get_smtp_port();
    let smtp_hello = opts.get_smtp_hello_name();

    match opts.use_ses() {
        true => {
            println!("Using Amazon SES to send email...");
            let ses_client = SesClient::new(rusoto_core::Region::default());
            let res = send_email_ses(&ses_client, &from, &to, &subject, body).await;
            if res.is_ok() {
                println!("Email sent successfully!")
            } else {
                println!("Error sending email: {:?}", res)
            }
        }
        false => {
            println!("Using SMTP to send email...");
            let smtp_credentials = Credentials::new(opts.get_smtp_user(), opts.get_smtp_password());
            let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?;
            let mailer = if let Some(p) = smtp_port{
                mailer.port(p)
            } else {
                mailer
            };
            let mailer = if let Some(p) = smtp_hello{
                mailer.hello_name(lettre::transport::smtp::extension::ClientId::Domain(p))
            } else {
                mailer
            };
            let mailer = mailer.credentials(smtp_credentials)
                .build();
            let res = send_email_smtp(&mailer, &from, &to, &subject, body).await;
            if res.is_ok() {
                println!("Email sent successfully!")
            } else {
                println!("Error sending email: {:?}", res)
            }
        }
    }
    Ok(())
}

/// Send Email using SMTP
async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}

/// Send Email using Amazon SES Service
async fn send_email_ses(
    ses_client: &SesClient,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let raw_email = email.formatted();

    let ses_request = SendRawEmailRequest {
        raw_message: RawMessage {
            data: general_purpose::STANDARD.decode(&raw_email).unwrap().into(),
        },
        ..Default::default()
    };

    ses_client.send_raw_email(ses_request).await?;

    Ok(())
}
