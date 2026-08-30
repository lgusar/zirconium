use std::{error::Error, fmt::Display};

mod join;
mod nick;
mod privmsg;
mod user;

pub use join::Join;
pub use join::JoinParams;
pub use nick::Nick;
pub use privmsg::PrivMsg;
pub use user::User;

#[derive(Debug)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}={}", self.key, value)
        } else {
            write!(f, "{}", self.key)
        }
    }
}

impl TryFrom<String> for Tag {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (key, value) = value.split_once("=").ok_or(ParseError::BadFormat)?;
        if value.is_empty() {
            Ok(Tag {
                key: key.to_string(),
                value: None,
            })
        } else {
            Ok(Tag {
                key: key.to_string(),
                value: Some(value.to_string()),
            })
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub user: Option<String>,
    pub host: Option<String>,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(user) = &self.user {
            write!(f, "{}", user)?;
        }
        if let Some(host) = &self.host {
            write!(f, "{}", host)?;
        }

        Ok(())
    }
}

impl TryFrom<String> for Source {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (rest, host) = value.split_once("@").ok_or(ParseError::BadFormat)?;
        let host = if host.is_empty() {
            None
        } else {
            Some(host.to_string())
        };

        let (name, user) = rest.split_once("!").ok_or(ParseError::BadFormat)?;
        let user = if user.is_empty() {
            None
        } else {
            Some(user.to_string())
        };

        Ok(Source {
            name: name.to_string(),
            user,
            host,
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    BadFormat,
    UnknownCommand(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadFormat => write!(f, "Bad format"),
            Self::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub enum Command {
    Join(Join),
    Nick(Nick),
    PrivMsg(PrivMsg),
    User(User),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Join(join) => join.fmt(f),
            Command::Nick(nick) => nick.fmt(f),
            Command::PrivMsg(privmsg) => privmsg.fmt(f),
            Command::User(user) => user.fmt(f),
        }
    }
}

impl TryFrom<String> for Command {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (command, parameters) = value.split_once(" ").ok_or(ParseError::BadFormat)?;
        match command {
            "JOIN" => Ok(Command::Join(Join::try_from(parameters.to_string())?)),
            "NICK" => Ok(Command::Nick(Nick::try_from(parameters.to_string())?)),
            "PRIVMSG" => Ok(Command::PrivMsg(PrivMsg::try_from(parameters)?)),
            "USER" => Ok(Command::User(User::try_from(parameters.to_string())?)),
            cmd => Err(ParseError::UnknownCommand(cmd.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub tags: Vec<Tag>,
    pub source: Option<Source>,
    pub command: Command,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.tags.is_empty() {
            let tags = self
                .tags
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(";");

            write!(f, "@{} ", tags)?;
        }

        if let Some(source) = &self.source {
            write!(f, ":{} ", source)?;
        }

        write!(f, "{}\r\n", self.command)
    }
}

impl Message {
    pub fn new(cmd: Command) -> Self {
        Message {
            tags: vec![],
            source: None,
            command: cmd,
        }
    }

    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags.extend(tags);
        self
    }

    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
}

impl TryFrom<String> for Message {
    type Error = ParseError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value = value.trim().to_string();

        let (tags, value) = if value.starts_with("@") {
            if let Some((tags, rest)) = value.split_once(" ") {
                let tags = tags.strip_prefix("@").ok_or(ParseError::BadFormat)?;
                (Some(tags.to_string()), rest.to_string())
            } else {
                return Err(ParseError::BadFormat);
            }
        } else {
            (None, value)
        };

        let (source, value): (Option<Source>, String) = if value.starts_with(":") {
            let (source, rest) = value.split_once(" ").ok_or(ParseError::BadFormat)?;
            let source = source.strip_prefix(":").ok_or(ParseError::BadFormat)?;
            (Some(source.to_string().try_into()?), rest.to_string())
        } else {
            (None, value)
        };

        let tags: Vec<Tag> = {
            if let Some(tags) = tags {
                tags.split(";")
                    .map(|t| t.to_string().try_into())
                    .collect::<Result<Vec<Tag>, _>>()?
            } else {
                Vec::new()
            }
        };

        let cmd = Command::try_from(value)?;
        let mut msg = Message::new(cmd).with_tags(tags);
        if let Some(source) = source {
            msg = msg.with_source(source);
        }

        Ok(msg)
    }
}
