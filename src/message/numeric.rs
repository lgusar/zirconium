use regex::regex;

use super::ParseError;

use std::fmt::Display;

#[derive(Debug)]
pub enum Numeric {
    RplWelcome {
        message: String,
    },
    RplYourHost {
        message: String,
    },
    RplCreated {
        message: String,
    },
    RplMyInfo {
        message: String,
    },
    RplISupport {
        message: String,
    }, // TODO: add support for extensions
    RplUModeIs {
        client: String,
        user_modes: String,
    },
    RplLUserClient {
        client: String,
        users: u32,
        invisible: u32,
        servers: u32,
    },
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::RplWelcome { message } => write!(f, "RPL_WELCOME {}", message),
            Numeric::RplYourHost { message } => write!(f, "RPL_YOURHOST {}", message),
            Numeric::RplCreated { message } => write!(f, "RPL_CREATED {}", message),
            Numeric::RplMyInfo { message } => write!(f, "RPL_MYINFO {}", message),
            Numeric::RplISupport { message } => write!(f, "RPL_ISUPPORT {}", message),
            Numeric::RplUModeIs { client, user_modes } => {
                write!(f, "RPL_UMODEIS {} {}", client, user_modes)
            }
            Numeric::RplLUserClient {
                client,
                users,
                invisible,
                servers,
            } => {
                write!(
                    f,
                    "RPL_LUSERCLIENT {} :There are {} users and {} invisible on {} servers",
                    client, users, invisible, servers
                )
            }
        }
    }
}

impl TryFrom<&str> for Numeric {
    type Error = ParseError;

    // INFO: this is the only TryFrom Command that has to accept both command and parameters,
    //       others need only parameters
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (command, parameters) = value
            .split_once(" ")
            .ok_or(ParseError::BadCommand(value.into()))?;
        match command {
            "001" => Ok(Numeric::RplWelcome {
                message: parameters.into(),
            }),
            "002" => Ok(Numeric::RplYourHost {
                message: parameters.into(),
            }),
            "003" => Ok(Numeric::RplCreated {
                message: parameters.into(),
            }),
            "004" => Ok(Numeric::RplMyInfo {
                message: parameters.into(),
            }),
            "005" => Ok(Numeric::RplISupport {
                message: parameters.into(),
            }),
            "221" => {
                let (client, user_modes) = parameters
                    .split_once(" ")
                    .ok_or(ParseError::BadCommand(value.into()))?;

                Ok(Numeric::RplUModeIs {
                    client: client.into(),
                    user_modes: user_modes.into(),
                })
            }
            "251" => {
                let re = regex!(
                    r"^(?<client>[^\x{0} \r\n:]*) :There are (?<users>\d+) users and (?<invisible>\d+) invisible on (?<servers>\d+) server\(?s\)?$"
                );

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(parameters.into()));
                };

                let client = caps
                    .name("client")
                    .map(|m| m.as_str().to_string())
                    .ok_or(ParseError::BadCommand("bad_client".into()))?;

                let users = caps
                    .name("users")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand("bad_users".into()))?
                    .map_err(|_| ParseError::BadCommand("bad_users".into()))?;

                let invisible = caps
                    .name("invisible")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand("bad_invisible".into()))?
                    .map_err(|_| ParseError::BadCommand("bad_invisible".into()))?;

                let servers = caps
                    .name("servers")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand("bad_servers".into()))?
                    .map_err(|_| ParseError::BadCommand("bad_servers".into()))?;

                Ok(Numeric::RplLUserClient {
                    client,
                    users,
                    invisible,
                    servers,
                })
            }
            _ => Err(ParseError::UnknownCommand(value.into())),
        }
    }
}

impl TryFrom<&String> for Numeric {
    type Error = ParseError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Numeric::try_from(value.as_str())
    }
}

impl TryFrom<String> for Numeric {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Numeric::try_from(value.as_str())
    }
}

// NOTE: numerics are only sent from the server, so we don't have to
// add support for creating them on the client
