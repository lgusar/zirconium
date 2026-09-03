use super::{Command, Message, ParseError};

use std::fmt::Display;

#[derive(Debug)]
pub struct PrivMsg {
    pub targets: Vec<String>,
    pub payload: String,
}

impl Display for PrivMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PRIVMSG {} :{}", self.targets.join(","), self.payload)
    }
}

impl From<PrivMsg> for Message {
    fn from(value: PrivMsg) -> Self {
        Message::new(Command::PrivMsg(value))
    }
}

impl TryFrom<&str> for PrivMsg {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((targets, mut payload)) = value.split_once(" ") {
            if payload.starts_with(":") {
                payload = payload.strip_prefix(":").ok_or(ParseError::BadCommand(
                    "expected to have ':' at the start of the payload".into(),
                    value.into(),
                ))?;
            }

            let targets: Vec<String> = targets.split(",").map(|t| t.to_string()).collect();

            Ok(PrivMsg {
                targets,
                payload: payload.to_string(),
            })
        } else {
            if value.is_empty() {
                Err(ParseError::BadCommand(
                    "received empty payload".into(),
                    value.into(),
                ))
            } else {
                let targets: Vec<String> = value.split(",").map(|t| t.to_string()).collect();
                Ok(PrivMsg {
                    targets,
                    payload: "".into(),
                })
            }
        }
    }
}

impl TryFrom<&String> for PrivMsg {
    type Error = ParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        PrivMsg::try_from(value.as_str())
    }
}

impl TryFrom<String> for PrivMsg {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PrivMsg::try_from(value.as_str())
    }
}
