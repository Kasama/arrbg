use std::net::Ipv4Addr;

use endian_trait::Endian;

use super::{Action, Event};

macro_rules! endian_func {
    ($value:tt, $fn:ident) => {
        fn $fn(mut self) -> Self {
            self.$value = self
                .$value
                .iter()
                .map(|byte| byte.$fn())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            self
        }
    };
}

macro_rules! impl_endian_for_byte_arr {
    ($type:ident, $bytes:tt) => {
        impl Endian for $type {
            endian_func!($bytes, to_be);
            endian_func!($bytes, to_le);
            endian_func!($bytes, from_be);
            endian_func!($bytes, from_le);
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
struct InfoHash([u8; 20]);
impl_endian_for_byte_arr!(InfoHash, 0);

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
struct PeerId([u8; 20]);
impl_endian_for_byte_arr!(PeerId, 0);

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
struct IpAddr([u8; 4]);
impl_endian_for_byte_arr!(IpAddr, 0);

impl From<IpAddr> for Ipv4Addr {
    fn from(value: IpAddr) -> Self {
        Ipv4Addr::from(value.from_be().0)
    }
}

impl From<Ipv4Addr> for IpAddr {
    fn from(value: Ipv4Addr) -> Self {
        Self(value.octets()).to_be()
    }
}

#[derive(Debug, Endian, PartialEq, Eq)]
#[repr(C)]
struct AnnounceInput {
    connection_id: u64,
    action: Action,
    transaction_id: u32,
    info_hash: InfoHash,
    peer_id: PeerId,
    downloaded: u64,
    left: u64,
    uploaded: u64,
    event: Event,
    ip: IpAddr,
    key: u32,
    num_want: i32,
    port: u16,
}

struct AnnounceOutput {
    action: Action,
    transaction_id: u32,
}
