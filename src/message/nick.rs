use super::{Command, Message, ParseError};

use std::fmt::Display;

#[derive(Debug)]
pub struct Nick {
    pub nickname: String,
}

impl Display for Nick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NICK {}", self.nickname)
    }
}

impl From<Nick> for Message {
    fn from(value: Nick) -> Self {
        Message::new(Command::Nick(value))
    }
}

// TODO: impl TryFrom<&String> for Nick
impl TryFrom<String> for Nick {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains(" ") || value.is_empty() {
            return Err(ParseError::BadCommand(value));
        }

        Ok(Nick { nickname: value })
    }
}
