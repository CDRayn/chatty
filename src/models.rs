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
    #[serde(default)]
    pub id: Option<&'a str>,
    pub timestamp: u32,
    pub message: &'a str,
    pub sourceUserId: u32,
    pub destinationUserId: u32,
}

/// Parses a Chat object from a request body.
///
/// # Parameters
///
/// - `http_body`: a reference to the `str` of the request body to parse a `Chat` object from.
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: A `Chat` struct containing the chat object posted by the client.
/// - `Err`: The error encountered when attempting to parse the request body.
pub fn parse_chat(http_body: &str) -> Result<Chat>
{
    let chat= serde_json::from_str(http_body);
    return chat;
}

/// Parses a Message object from a request body.
///
/// # Parameters
///
/// - `http_body`: A reference to the `str` of the request body to parse a `Message` object from.
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: A `Message` struct containing the message object posted by the client.
/// - `Err`: The error encountered when attempting to parse the request body.
pub fn parse_message(http_body: &str) -> Result<Message>
{
    let message = serde_json::from_str(http_body);
    return message;
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

    /// Verify that the `parse_chat()` returns an error when parsing an incorrectly
    /// JSON formatted chat object.
    #[test]
    fn test_parse_chat_invalid()
    {
        // Test the parsing of a chat object that is missing the participantIds field.
        let mut json_chat = r#"
            {
                "id": 34
            }
        "#;
        let mut result = parse_chat(&json_chat).is_err();
        assert!(result);

        // Test the parsing of a chat object that is not valid JSON
        json_chat = r#"
            {
                "id: 34,
                "participantIds": [3423, 9813]
            }
        "#;
        result = parse_chat(&json_chat).is_err();
        assert!(result);

        json_chat = r#"
            {
                "id": 34,
                "participantIds": [3423, 9813],
            }
        "#;
        result = parse_chat(&json_chat).is_err();
        assert!(result);

        json_chat = r#"
            {
                id: 34,
                "participantIds": [3423, 9813]
            }
        "#;
        result = parse_chat(&json_chat).is_err();
        assert!(result);

        json_chat = r#"
            {
                "id": 34,
                participantIds: [3423, 9813]
            }
        "#;
        result = parse_chat(&json_chat).is_err();
        assert!(result);
    }

    /// Verify that the `parse_message()` function correctly parses a `Message` struct from
    /// a JSON formatted HTTP body.
    #[test]
    fn test_parse_message_valid()
    {
        let mut json_message = r#"
            {
                "id": "8911889c-8b93-4786-bbf3-50d56868b309",
                "timestamp": 1572297339,
                "message": "snake_case is more readable than CamelCase!",
                "sourceUserId": 9837,
                "destinationUserId": 1983
            }
        "#;
        let mut expected = Message {
            id: Some("8911889c-8b93-4786-bbf3-50d56868b309"),
            timestamp: 1572297339,
            message: "snake_case is more readable than CamelCase!",
            sourceUserId: 9837,
            destinationUserId: 1983,
        };
        let mut parsed_message = parse_message(&json_message).unwrap();

        assert_eq!(expected.id, parsed_message.id);
        assert_eq!(expected.timestamp, parsed_message.timestamp);
        assert_eq!(expected.message, parsed_message.message);
        assert_eq!(expected.sourceUserId, parsed_message.sourceUserId);
        assert_eq!(expected.destinationUserId, parsed_message.destinationUserId);
    }
}