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
    RplLUserOp {
        client: String,
        operators: u32,
    },
    RplLUserUnknown {
        client: String,
        connections: u32,
    },
    RplLUserChannels {
        client: String,
        channels: u32,
    },
    RplLUserMe {
        client: String,
        clients: u32,
        servers: u32,
    },
    RplLocalUsers {
        client: String,
        current: u32,
        maximum: u32,
    },
    RplGlobalUsers {
        client: String,
        current: u32,
        maximum: u32,
    },
    RplMotd {
        message: String,
    },
    RplMotdStart {
        message: String,
    },
    RplEndOfMotd {
        message: String,
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
            Numeric::RplLUserOp { client, operators } => {
                write!(
                    f,
                    "RPL_LUSEROP {} {} :operator(s) online",
                    client, operators
                )
            }
            Numeric::RplLUserUnknown {
                client,
                connections,
            } => {
                write!(
                    f,
                    "RPL_LUSERUNKNOWN {} {} :unknown connection(s)",
                    client, connections
                )
            }
            Numeric::RplLUserChannels { client, channels } => {
                write!(
                    f,
                    "RPL_LUSERCHANNELS {} {} :channels formed",
                    client, channels
                )
            }
            Numeric::RplLUserMe {
                client,
                clients,
                servers,
            } => {
                write!(
                    f,
                    "RPL_LUSERME {}: I have {} clients and {} servers",
                    client, clients, servers
                )
            }
            Numeric::RplLocalUsers {
                client,
                current,
                maximum,
            } => {
                write!(
                    f,
                    "RPL_LOCALUSERS {} [{} {}] :Current local users {}, max {}",
                    client, current, maximum, current, maximum
                )
            }
            Numeric::RplGlobalUsers {
                client,
                current,
                maximum,
            } => {
                write!(
                    f,
                    "RPL_GLOBALUSERS {} [{} {}] :Current global users {}, max {}",
                    client, current, maximum, current, maximum
                )
            }
            Numeric::RplMotd { message } => write!(f, "RPL_MOTD {}", message),
            Numeric::RplMotdStart { message } => write!(f, "RPL_MOTDSTART {}", message),
            Numeric::RplEndOfMotd { message } => write!(f, "RPL_ENDOFMOTD {}", message),
        }
    }
}

impl TryFrom<&str> for Numeric {
    type Error = ParseError;

    // INFO: this is the only TryFrom Command that has to accept both command and parameters,
    //       others need only parameters
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (command, parameters) = value.split_once(" ").ok_or(ParseError::BadCommand(
            "expected command and parameters".into(),
            value.into(),
        ))?;
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
                let (client, user_modes) = parameters.split_once(" ").ok_or(
                    ParseError::BadCommand("expected client and user modes".into(), value.into()),
                )?;

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
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let users = caps
                    .name("users")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected users count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected users count".into(), value.into())
                    })?;

                let invisible = caps
                    .name("invisible")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "bad_invisible".into(),
                        parameters.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("bad_invisible".into(), parameters.into())
                    })?;

                let servers = caps
                    .name("servers")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "bad_servers".into(),
                        parameters.into(),
                    ))?
                    .map_err(|_| ParseError::BadCommand("bad_servers".into(), parameters.into()))?;

                Ok(Numeric::RplLUserClient {
                    client,
                    users,
                    invisible,
                    servers,
                })
            }
            "252" => {
                let re = regex!(r"^(?<client>[^\x{0} \r\n:]*) (?<ops>\d+) :.*$");

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let operators = caps
                    .name("ops")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected operators count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected operators count".into(), value.into())
                    })?;

                Ok(Numeric::RplLUserOp { client, operators })
            }
            "253" => {
                let re = regex!(r"^(?<client>[^\x{0} \r\n:]*) (?<connections>\d+) :.*$");

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let connections = caps
                    .name("connections")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected connections count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected connections count".into(), value.into())
                    })?;

                Ok(Numeric::RplLUserUnknown {
                    client,
                    connections,
                })
            }
            "254" => {
                let re = regex!(r"^(?<client>[^\x{0} \r\n:]*) (?<channels>\d+) :.*$");

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let channels = caps
                    .name("channels")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected channels count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected channels count".into(), value.into())
                    })?;

                Ok(Numeric::RplLUserChannels { client, channels })
            }
            "255" => {
                let re = regex!(
                    r"^(?<client>[^\x{0} \r\n:]*) :I have (?<clients>\d+) clients and (?<servers>\d+) servers$"
                );

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let clients = caps
                    .name("clients")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected clients count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected clients count".into(), value.into())
                    })?;

                let servers = caps
                    .name("servers")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected servers count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected servers count".into(), value.into())
                    })?;

                Ok(Numeric::RplLUserMe {
                    client,
                    clients,
                    servers,
                })
            }
            "265" => {
                let re = regex!(
                    r"^(?<client>[^\x{0} \r\n:]*) (?<current>\d+) (?<maximum>\d+) :Current local users \d+, max \d+$"
                );

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let current = caps
                    .name("current")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected current count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected current count".into(), value.into())
                    })?;

                let maximum = caps
                    .name("maximum")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected maximum count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected maximum count".into(), value.into())
                    })?;

                Ok(Numeric::RplLocalUsers {
                    client,
                    current,
                    maximum,
                })
            }
            "266" => {
                let re = regex!(
                    r"^(?<client>[^\x{0} \r\n:]*) (?<current>\d+) (?<maximum>\d+) :Current global users \d+, max \d+$"
                );

                let Some(caps) = re.captures(parameters) else {
                    return Err(ParseError::BadCommand(
                        "failed to parse parameters".into(),
                        value.into(),
                    ));
                };

                let client = caps.name("client").map(|m| m.as_str().to_string()).ok_or(
                    ParseError::BadCommand("expected client".into(), value.into()),
                )?;

                let current = caps
                    .name("current")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected current count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected current count".into(), value.into())
                    })?;

                let maximum = caps
                    .name("maximum")
                    .map(|m| m.as_str().parse::<u32>())
                    .ok_or(ParseError::BadCommand(
                        "expected maximum count".into(),
                        value.into(),
                    ))?
                    .map_err(|_| {
                        ParseError::BadCommand("expected maximum count".into(), value.into())
                    })?;

                Ok(Numeric::RplGlobalUsers {
                    client,
                    current,
                    maximum,
                })
            }
            "372" => Ok(Numeric::RplMotd {
                message: parameters.into(),
            }),
            "375" => Ok(Numeric::RplMotdStart {
                message: parameters.into(),
            }),
            "376" => Ok(Numeric::RplEndOfMotd {
                message: parameters.into(),
            }),
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
