use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use serde_json::json;

/// Represents a Discord webhook message.
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordWebhookMessage {
    /// The URL of the Discord webhook.
    webhook_url: String,
}

impl DiscordWebhookMessage {
    /// Creates a new instance of `DiscordWebhookMessage`.
    ///
    /// # Arguments
    ///
    /// * `webhook_url` - The URL of the Discord webhook.
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordWebhookMessage`.
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }

    /// Sends a message to the Discord webhook.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the message.
    /// * `description` - The description of the message.
    /// * `content` - The content of the message.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_discord_webhook::{DiscordWebhookMessage};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let webhook_url = "https://discord.com/api/webhooks/your-webhook-url";
    ///     let message = DiscordWebhookMessage::new(webhook_url.to_string());
    ///
    ///     let title = "Hello";
    ///     let description = "This is a test message";
    ///     let content = "Hello from Rust!";
    ///
    ///     match message.send(title, description, content).await {
    ///         Ok(_) => println!("Message sent successfully!"),
    ///         Err(err) => eprintln!("Failed to send message: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn send(
        &self,
        title: &str,
        description: &str,
        content: &str,
    ) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let message = json!({
            "content": content,
            "embeds": [
                {
                    "title": title,
                    "description": description,
                    "color": 65280 // Green color
                }
            ]
        });

        let response = client
            .post(&self.webhook_url)
            .header("Content-Type", "application/json")
            .json(&message)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            println!("Message sent successfully!");
        } else {
            println!("Failed to send message: {}", response.status());
        }

        Ok(())
    }
}
