pub mod connect;
pub mod announce;

use std::net::Ipv4Addr;

use endian_trait::Endian;

const CONNECTION_ID: u64 = 0x41727101980;

#[derive(Debug, Endian, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Action {
    Connect = 0,
    Announce = 1,
    Scrape = 2,
    Error = 3,
}

#[derive(Debug, Endian, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Event {
    None = 0,
    Completed = 1,
    Started = 2,
    Stopped = 3,
}
