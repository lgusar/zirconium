use super::{Command, Message};

use std::fmt::Display;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub realname: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "USER {} 0 * {}", self.username, self.realname)
    }
}

impl From<User> for Message {
    fn from(value: User) -> Self {
        Message::new(Command::User(value))
    }
}

// TODO: implement TryFrom for User for String and &String
// impl TryFrom<String> for Nick {
//     type Error = ParseError;
//
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         if value.contains(" ") || value.is_empty() {
//             return Err(ParseError::BadFormat);
//         }
//
//         Ok(Nick { nickname: value })
//     }
// }
