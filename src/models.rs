#![allow(non_snake_case)]
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Result;

/// # Chat Struct
///
/// Struct that represents a chat session between two users
/// `id`: The Chat's ID
/// `participants_ids`: The unique ids of the two participants.
#[derive(Serialize, Deserialize)]
pub struct Chat
{
    #[serde(default)]
    pub id: Option<u32>,
    pub participantIds: [u32; 2],
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
    pub sourceUserId: u32,
    pub destinationUserId: u32,
    pub timestamp: u32,
    pub message: &'a str,
}

/// Parses an HTTP body and returns a `Chat` struct containing the parsed contents
/// of the HTTP body.
pub fn parse_chat(http_body: &str) -> Result<Chat>
{


    let chat= serde_json::from_str(http_body);
    return chat;
}

#[cfg(test)]
mod test
{
    use super::*;

    /// Verify that the `parse_chat()` function correctly parses a `Chat` struct from
    /// a JSON formatted HTTP body.
    #[test]
    fn test_parse_chat_valid()
    {
        // Test the parsing of a JSON formatted chat object containing all fields
        let mut json_chat = r#"
            {
                "id": 34,
                "participantIds": [3423, 9813]
            }
        "#;
        let mut expected = Chat {
            id: Some(34),
            participantIds: [3423, 9813],
        };
        let mut parsed_chat = parse_chat(&json_chat).unwrap();

        assert_eq!(expected.id, parsed_chat.id);
        assert_eq!(expected.participantIds[0], parsed_chat.participantIds[0]);
        assert_eq!(expected.participantIds[1], parsed_chat.participantIds[1]);

        // Test the parsing of a JSON formatted chat object that does
        // not contain the id field.
        json_chat = r#"
            {
                "participantIds": [3423, 9813]
            }
        "#;
        expected = Chat {
            id: None,
            participantIds: [3423, 9813],
        };
        parsed_chat = parse_chat(&json_chat).unwrap();

        assert_eq!(expected.id, parsed_chat.id);
        assert_eq!(expected.participantIds[0], parsed_chat.participantIds[0]);
        assert_eq!(expected.participantIds[1], parsed_chat.participantIds[1]);
    }
}