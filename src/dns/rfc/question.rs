use std::fmt;

use type2network::{FromNetworkOrder, ToNetworkOrder};
use type2network_derive::{FromNetwork, ToNetwork};

use serde::Serialize;

use crate::dns::rfc::{domain::DomainName, qclass::QClass, qtype::QType};
use crate::TITLES;

// Question structure: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2
// 1  1  1  1  1  1
// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                                               |
// /                     QNAME                     /
// /                                               /
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                     QTYPE                     |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                     QCLASS                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug, Default, Clone, PartialEq, ToNetwork, FromNetwork, Serialize)]
pub struct Question {
    pub qname: DomainName,
    pub qtype: QType,
    pub qclass: QClass,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}:{:?} {}:{:?}",
            TITLES["qname"], self.qname, TITLES["qtype"], self.qtype, TITLES["qclass"], self.qclass
        )
    }
}

#[cfg(test)]
mod tests {
    use type2network::{FromNetworkOrder, ToNetworkOrder};

    #[test]
    fn network() {
        use crate::dns::{rfc::qclass::QClass, rfc::qtype::QType, rfc::question::Question};
        use std::io::Cursor;

        let sample = vec![
            0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01,
            0x00, 0x01,
        ];
        let mut buffer = Cursor::new(sample.as_slice());
        let mut q = Question::default();
        assert!(q.deserialize_from(&mut buffer).is_ok());
        assert_eq!(q.qname.to_string(), "www.google.com.");
        assert_eq!(q.qtype, QType::A);
        assert_eq!(q.qclass, QClass::IN);

        let mut buffer: Vec<u8> = Vec::new();
        assert!(q.serialize_to(&mut buffer).is_ok());
        assert_eq!(buffer, sample);
    }
}
