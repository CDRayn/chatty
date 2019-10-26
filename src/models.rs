use serde::{Deserialize, Serialize};
use serde_json::{Result, from_str};
use std::error::Error;

/// # Chat Struct
///
/// Struct that represents a chat session between two users
/// `id`: The Chat's ID
/// `participants_ids`: The unique ids of the two participants.
#[derive(Serialize, Deserialize)]
pub struct Chat
{
    id: u32,
    participant_ids: [u32; 2],
}

/// # Message Struct
///
/// Struct that represents a message sent via a chat session between two users.
/// `source_user_id`: The sender's user ID
/// `destination_user_id`: The recipient's user ID
/// `timestamp`: The epoch millis that correspond with when the message was sent.
/// `message`: The body of the message.
#[derive(Serialize, Deserialize)]
pub struct Message<'a>
{
    source_user_id: u32,
    destination_user_id: u32,
    timestamp: u32,
    message: &'a str,
}

/// Parses an HTTP body and returns a `Chat` struct containing the parsed contents
/// of the HTTP body.
pub fn parse_chat(http_body: &str) -> Result<Chat>
{
    let chat: Chat = serde_json::from_str(http_body)?;
    return Ok(chat)
}
