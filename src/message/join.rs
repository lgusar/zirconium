use super::{Command, Message, ParseError};

use std::fmt::Display;

#[derive(Debug)]
pub enum JoinParams {
    Channels {
        channels: Vec<String>,
        keys: Vec<String>,
    },
    Leave,
}

#[derive(Debug)]
pub struct Join {
    pub params: JoinParams,
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JOIN")?;
        match &self.params {
            JoinParams::Leave => write!(f, " 0"),
            JoinParams::Channels { channels, keys } => {
                if keys.is_empty() {
                    write!(f, " {}", channels.join(","))
                } else {
                    write!(f, " {} {}", channels.join(","), keys.join(","))
                }
            }
        }
    }
}

impl From<Join> for Message {
    fn from(value: Join) -> Self {
        Message::new(Command::Join(value))
    }
}

impl TryFrom<&String> for Join {
    type Error = ParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value == "0" {
            Ok(Join {
                params: JoinParams::Leave,
            })
        } else {
            if value.contains(" ") {
                let (channels, keys) = value.split_once(" ").ok_or(ParseError::BadFormat)?;
                let channels: Vec<String> = channels.split(",").map(|c| c.to_string()).collect();
                let keys: Vec<String> = keys.split(",").map(|c| c.to_string()).collect();

                if channels.len() != keys.len() {
                    // TODO: change error message
                    return Err(ParseError::BadFormat);
                }

                Ok(Join {
                    params: JoinParams::Channels { channels, keys },
                })
            } else {
                let channels: Vec<String> = value.split(",").map(|c| c.to_string()).collect();

                Ok(Join {
                    params: JoinParams::Channels {
                        channels,
                        keys: vec![],
                    },
                })
            }
        }
    }
}

impl TryFrom<String> for Join {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Join::try_from(&value)
    }
}
