use endian_trait::Endian;

use super::{Action, CONNECTION_ID};

#[doc = r"spec described at at: https://xbtt.sourceforge.net/udp_tracker_protocol.html

               connect input
Offset	Size    Type    Name                    Value
0       64-bit  integer connection_id   0x41727101980
8       32-bit  integer	action                      0
12      32-bit  integer	transaction_id
16"]
#[derive(Debug, Endian, PartialEq, Eq)]
#[repr(C)]
struct ConnectInput {
    connection_id: u64,
    action: Action,
    pub(crate) transaction_id: u32,
}

type ConnectInputPacket = [u8; 16];

impl ConnectInput {
    fn new(transaction_id: u32) -> Self {
        Self {
            connection_id: CONNECTION_ID,
            action: Action::Connect,
            transaction_id,
        }
    }
}

impl From<ConnectInput> for ConnectInputPacket {
    fn from(val: ConnectInput) -> Self {
        unsafe { std::mem::transmute(val.to_be()) }
    }
}

#[doc = r"spec described at at: https://xbtt.sourceforge.net/udp_tracker_protocol.html

               connect output
Offset	Size    Type    Name                    Value
0       32-bit  integer	action                      0
4       32-bit  integer	transaction_id
8       64-bit  integer connection_id
16"]
#[derive(Debug, Endian, PartialEq, Eq)]
#[repr(C)]
struct ConnectOutput {
    action: Action,
    transaction_id: u32,
    connection_id: u64,
}

type ConnectOutputPacket = [u8; 16];

impl From<ConnectOutputPacket> for ConnectOutput {
    fn from(value: ConnectOutputPacket) -> Self {
        let v: ConnectOutput = unsafe { std::mem::transmute(value) };
        v.from_be()
    }
}

#[cfg(test)]
mod tests {
    use super::{ConnectInput, ConnectOutput, CONNECTION_ID};
    use crate::torrent::udp::connect::ConnectInputPacket;
    use crate::torrent::udp::{connect::ConnectOutputPacket, Action};
    use byteorder::{BigEndian, ByteOrder};
    use std::mem::zeroed;

    #[test]
    fn test_connect_input() {
        let action: Action = Action::Connect;
        let transaction_id: u32 = 72;

        let connect_input = ConnectInput::new(transaction_id);

        // Following spec at: https://xbtt.sourceforge.net/udp_tracker_protocol.html
        let mut expected: ConnectInputPacket = unsafe { zeroed() };
        BigEndian::write_u64(&mut expected[0..8], CONNECTION_ID);
        BigEndian::write_u32(&mut expected[8..12], action as u32);
        BigEndian::write_u32(&mut expected[12..16], transaction_id);

        let actual: ConnectInputPacket = connect_input.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_connect_output() {
        let action: Action = Action::Connect;
        let transaction_id: u32 = 42;

        // Following spec at: https://xbtt.sourceforge.net/udp_tracker_protocol.html
        let mut buf: ConnectOutputPacket = unsafe { zeroed() };
        BigEndian::write_u32(&mut buf[0..4], action as u32);
        BigEndian::write_u32(&mut buf[4..8], transaction_id);
        BigEndian::write_u64(&mut buf[8..16], CONNECTION_ID);

        let expected = ConnectOutput {
            action,
            transaction_id,
            connection_id: CONNECTION_ID,
        };

        assert_eq!(expected, buf.into());
    }

    #[test]
    fn connect_to_tracker() {
    }
}
