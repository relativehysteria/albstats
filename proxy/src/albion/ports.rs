/// Ports as utilized by the Albion servers
#[derive(Copy, Clone, Debug)]
#[repr(u16)]
#[allow(dead_code)]
enum NetworkPort {
    /// Chats such as faction, alliance, etc.
    GlobalChats = 4535,

    // Truth is I don't actually know what the difference between these two
    // ports is so their name is probably misleading, but that's okay because
    // they're not pub anyway.

    GlobalGame = 5055,
    LocalGame = 5056,
}

/// Only packets coming from ports in this filter pass through the proxy
pub const FILTER: [u16; 2] = [
    NetworkPort::GlobalGame as u16,
    NetworkPort::LocalGame as u16,
];
