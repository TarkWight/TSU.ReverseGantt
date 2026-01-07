use anyhow::Context;
use lettre::{
    message::{header::ContentType, Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use std::env;

pub struct SmtpService {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
    from_name: String,
}

impl SmtpService {
    pub fn new() -> Self {
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_default();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
        
        if smtp_username.is_empty() || smtp_password.is_empty() {
            tracing::warn!(
                "SMTP credentials not found. Email notifications disabled. \
                Current dir: {:?}",
                std::env::current_dir().ok()
            );
        } else {
            tracing::info!("SMTP configured with username: {}", smtp_username);
        }
        
        Self {
            smtp_server: env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_username,
            smtp_password,
            from_email: env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@reverse-gantt.local".to_string()),
            from_name: env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "Reverse Gantt System".to_string()),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.smtp_username.is_empty() && !self.smtp_password.is_empty()
    }

    pub async fn send_task_notification(
        &self,
        to_email: &str,
        to_name: &str,
        task_name: &str,
        old_status: Option<&str>,
        new_status: &str,
        review_comment: Option<&str>,
        is_approved: bool,
        is_rejected: bool,
    ) -> anyhow::Result<()> {
        if !self.is_configured() {
            tracing::warn!("SMTP not configured, skipping email notification");
            return Ok(());
        }

        let (subject, body) = self.build_notification_content(
            task_name,
            old_status,
            new_status,
            review_comment,
            is_approved,
            is_rejected,
        );

        self.send_email(to_email, to_name, &subject, &body).await
    }

    pub async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        body: &str,
    ) -> anyhow::Result<()> {
        let from: Mailbox = format!("{} <{}>", self.from_name, self.from_email)
            .parse()
            .context("Invalid from email")?;
        let to: Mailbox = format!("{} <{}>", to_name, to_email)
            .parse()
            .context("Invalid to email")?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(self.format_html_email(body)),
                    ),
            )
            .context("Failed to build email message")?;

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = if self.smtp_port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.smtp_server)
                .context("Failed to create SMTP implicit TLS relay")?
                .port(self.smtp_port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_server)
                .context("Failed to create SMTP STARTTLS relay")?
                .credentials(creds)
                .build()
        };

        match mailer.send(email).await {
            Ok(response) => {
                tracing::info!("Email sent to {}: {:?}", to_email, response);
                Ok(())
            }
            Err(e) => {
                tracing::error!("SMTP error sending to {}: {:?}", to_email, e);
                Err(anyhow::anyhow!("Failed to send email: {}", e))
            }
        }
    }

    fn build_notification_content(
        &self,
        task_name: &str,
        old_status: Option<&str>,
        new_status: &str,
        review_comment: Option<&str>,
        is_approved: bool,
        is_rejected: bool,
    ) -> (String, String) {
        let subject = format!("Task Update: {}", task_name);

        let mut body = format!("Hello!\n\nYour task \"{}\" has been updated.\n\n", task_name);

        if let Some(old) = old_status {
            body.push_str(&format!("Status changed: {} → {}\n\n", old, new_status));
        } else {
            body.push_str(&format!("New status: {}\n\n", new_status));
        }

        if is_approved {
            body.push_str("🎉 Congratulations! Your task has been approved!\n\n");
            body.push_str("Fantastic work! Your task has been reviewed and accepted.\n\n");
        } else if is_rejected {
            body.push_str("📝 Your task has been sent back for revision.\n\n");
            if let Some(comment) = review_comment {
                body.push_str("Reviewer's feedback:\n");
                body.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                body.push_str(&format!("{}\n", comment));
                body.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");
            }
            body.push_str("Don't worry, you've got this! 💪\n\n");
        }

        body.push_str("Best regards,\nReverse Gantt System");

        (subject, body)
    }

    fn format_html_email(&self, text: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #0d6efd; color: white; padding: 20px; border-radius: 5px 5px 0 0; }}
        .content {{ background: #f8f9fa; padding: 20px; border-radius: 0 0 5px 5px; }}
        .footer {{ margin-top: 20px; font-size: 12px; color: #6c757d; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header"><h2>Reverse Gantt System</h2></div>
        <div class="content">{}</div>
        <div class="footer">This is an automated notification.</div>
    </div>
</body>
</html>"#,
            text.replace('\n', "<br>")
        )
    }
}

impl Default for SmtpService {
    fn default() -> Self {
        Self::new()
    }
}

