use super::{Command, Message, ParseError};

use std::fmt::Display;

use regex::regex;

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

impl TryFrom<&str> for User {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = regex!(r"(\w*) ?0? ?\*? (\w*)");
        let (_, [username, realname]) = re
            .captures(value)
            .map(|caps| caps.extract())
            .ok_or(ParseError::BadFormat)?;

        Ok(User {
            username: username.into(),
            realname: realname.into(),
        })
    }
}

impl TryFrom<&String> for User {
    type Error = ParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        User::try_from(value.as_str())
    }
}

impl TryFrom<String> for User {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        User::try_from(value.as_str())
    }
}
