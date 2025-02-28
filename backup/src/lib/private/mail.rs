// Copyright (c) 2012 Frank Karlitschek <frank@owncloud.org>
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

/// A module to handle mail sending.
use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, client::Tls},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use log::{debug, error};
use std::time::Duration;

use crate::config::OcConfig;
use crate::defaults::OcDefaults;

pub struct OcMail;

impl OcMail {
    /// Send an email
    ///
    /// # Arguments
    ///
    /// * `to_address` - Recipient's email address
    /// * `to_name` - Recipient's name
    /// * `subject` - Email subject
    /// * `mail_text` - Email body text
    /// * `from_address` - Sender's email address
    /// * `from_name` - Sender's name
    /// * `html` - Whether the email is HTML or not
    /// * `alt_body` - Alternative body text
    /// * `cc_address` - CC recipient's email address
    /// * `cc_name` - CC recipient's name
    /// * `bcc` - BCC recipient's email address
    ///
    pub async fn send(
        to_address: &str,
        to_name: &str,
        subject: &str,
        mail_text: &str,
        from_address: &str,
        from_name: &str,
        html: bool,
        alt_body: &str,
        cc_address: &str,
        cc_name: &str,
        bcc: &str,
    ) -> Result<()> {
        let smtp_mode = OcConfig::get_value("mail_smtpmode", "sendmail");
        let smtp_host = OcConfig::get_value("mail_smtphost", "127.0.0.1");
        let smtp_port = OcConfig::get_value("mail_smtpport", 25);
        let smtp_auth = OcConfig::get_value("mail_smtpauth", false);
        let smtp_auth_type = OcConfig::get_value("mail_smtpauthtype", "LOGIN");
        let smtp_username = OcConfig::get_value("mail_smtpname", "");
        let smtp_password = OcConfig::get_value("mail_smtppassword", "");
        let smtp_debug = OcConfig::get_value("mail_smtpdebug", false);
        let smtp_timeout = OcConfig::get_value("mail_smtptimeout", 10);
        let smtp_secure = OcConfig::get_value("mail_smtpsecure", "");

        // Parse addresses to mailboxes
        let from = Mailbox::new(Some(from_name.to_string()), from_address.parse()?);
        
        // Build message
        let mut builder = Message::builder()
            .from(from.clone())
            .reply_to(from)
            .subject(subject);

        // Add recipients
        for addr in to_address.split_whitespace() {
            let to = Mailbox::new(Some(to_name.to_string()), addr.parse()?);
            builder = builder.to(to);
        }

        // Add CC if provided
        if !cc_address.is_empty() {
            let cc = Mailbox::new(Some(cc_name.to_string()), cc_address.parse()?);
            builder = builder.cc(cc);
        }

        // Add BCC if provided
        if !bcc.is_empty() {
            builder = builder.bcc(bcc.parse()?);
        }

        // Create email body with footer
        let body_text = if alt_body.is_empty() {
            format!("{}{}", mail_text, Self::get_footer())
        } else {
            mail_text.to_string()
        };

        // Create email with proper content type
        let email = if html {
            if alt_body.is_empty() {
                builder.multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_PLAIN)
                                .body(body_text.clone()),
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_HTML)
                                .body(body_text),
                        ),
                )
            } else {
                builder.multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_PLAIN)
                                .body(alt_body.to_string()),
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_HTML)
                                .body(mail_text.to_string()),
                        ),
                )
            }
        } else {
            builder.body(body_text)
        }?;

        // Configure SMTP transport based on settings
        let mut transport_builder = match smtp_mode.as_str() {
            "smtp" => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host),
            _ => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host),
        };

        // Configure SMTP transport options
        transport_builder = transport_builder
            .port(smtp_port)
            .timeout(Some(Duration::from_secs(smtp_timeout)));

        // Configure TLS if needed
        if !smtp_secure.is_empty() {
            match smtp_secure.as_str() {
                "ssl" => transport_builder = transport_builder.tls(Tls::Wrapper),
                "tls" => transport_builder = transport_builder.tls(Tls::Required),
                _ => {}
            }
        }

        // Configure authentication if needed
        if smtp_auth {
            let credentials = Credentials::new(smtp_username, smtp_password);
            transport_builder = transport_builder.credentials(credentials);
        }

        // Build the transport
        let transport = transport_builder.build();

        // Send the email
        transport
            .send(email)
            .await
            .context("Failed to send email")?;

        debug!(
            "Mail from {} ({}) to: {}({}) subject: {}",
            from_name, from_address, to_name, to_address, subject
        );

        Ok(())
    }

    /// Return the footer for a mail
    pub fn get_footer() -> String {
        let defaults = OcDefaults::new();

        format!("\n--\n{}\n{}\n", defaults.get_name(), defaults.get_slogan())
    }

    /// Validate an email address
    ///
    /// # Arguments
    ///
    /// * `email_address` - a given email address to be validated
    pub fn validate_address(email_address: &str) -> bool {
        // Using a simple regex for validation - in production code you'd want a more robust check
        // or use a dedicated crate like email-address-parser
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email_address)
    }
}