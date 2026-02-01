use async_trait::async_trait;

#[async_trait]
pub trait CodeSender: Send + Sync {
    async fn send_code(&self, to: &str, code: &str) -> Result<(), anyhow::Error>;
}

pub struct ConsoleSender;

#[async_trait]
impl CodeSender for ConsoleSender {
    async fn send_code(&self, to: &str, code: &str) -> Result<(), anyhow::Error> {
        println!("[MOCK] Sending code {} to {}", code, to);
        Ok(())
    }
}
