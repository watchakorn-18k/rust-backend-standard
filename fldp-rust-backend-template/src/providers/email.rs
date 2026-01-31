#[derive(Clone)]
pub struct EmailProvider;

impl EmailProvider {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        // Placeholder for lettre or other email service
        println!("Sending email to: {}, Subject: {}, Body: {}", to, subject, body);
        Ok(())
    }
}
