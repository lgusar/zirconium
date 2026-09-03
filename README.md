# Zirconium

*Work in progress* IRC client written in Rust.

No LLMs were used in writing this code.

## Roadmap

Current goal is to have an application that can be used to connect to a server
(no TLS for now), and join a channel. Obviously this can be implemented very
trivially for an IRC client, so another constraint is that a user must be able
to join at least two channels on one server and have the two streams separate in
a meaningful way (definition of meaningful way is that a user can easily
understand what message is from what channel). Also, a user has to be able to
communicate with other users.

## Resources

The [Modern IRC Client Protocol](https://modern.ircdocs.horse/) is kind of a
holy grail if you want to implement IRC yourself, both a client and a server.
I did notice some differences between different IRC server implementations and
with what is written in the document.
