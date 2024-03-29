use crate::constants::{EMAIL_BODY, EMAIL_SUBJECT};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "Send test Email!")]
#[clap(disable_help_flag = true)]
pub(crate) struct CliOpts {
    /// SMTP Host
    #[clap(short = 'h', long, value_name = "SMTP-HOST")]
    smtp_host: String,
    /// SMTP User
    #[clap(short = 'u', long, value_name = "SMTP-USER")]
    smtp_user: String,
    /// SMTP Password
    #[clap(short = 'p', long, value_name = "SMTP-PASSWORD")]
    smtp_password: String,
    /// From Email
    #[clap(short = 'f', long, value_name = "FROM-EMAIL")]
    from_email: String,
    /// To Email
    #[clap(short = 't', long, value_name = "TO-EMAIL")]
    to_email: String,
    /// Default is None, if none is provided, it will use the default email subject
    #[clap(short = 's', long, value_name = "SUBJECT", default_value = None)]
    subject: Option<String>,
    /// Default is None, if none is provided, it will use the default email body
    #[clap(short = 'b', long, value_name = "BODY", default_value = None)]
    body: Option<String>,
    /// USE Amazon SES Service, if yes!
    #[clap(long, default_value = "false")]
    use_ses: bool,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

impl CliOpts {
    /// Parse the CLI Command
    pub fn parse_cli() -> Self {
        CliOpts::parse()
    }

    /// Get the Email Subject
    pub(crate) fn get_subject(&self) -> String {
        self.subject
            .clone()
            .unwrap_or_else(|| EMAIL_SUBJECT.to_string())
    }

    /// Get the Email Body
    pub(crate) fn get_body(&self) -> String {
        self.body.clone().unwrap_or_else(|| EMAIL_BODY.to_string())
    }

    /// Get the SMTP Host
    pub(crate) fn get_smtp_host(&self) -> String {
        self.smtp_host.to_owned()
    }

    /// Get the SMTP User
    pub(crate) fn get_smtp_user(&self) -> String {
        self.smtp_user.to_owned()
    }

    /// Get the SMTP Password
    pub(crate) fn get_smtp_password(&self) -> String {
        self.smtp_password.to_owned()
    }

    /// Get the From Email
    pub(crate) fn get_from_email(&self) -> String {
        self.from_email.to_owned()
    }

    /// Get the To Email
    pub(crate) fn get_to_email(&self) -> String {
        self.to_email.to_owned()
    }

    /// Use Amazon SES
    pub(crate) fn use_ses(&self) -> bool {
        self.use_ses
    }
}
