use std::fmt;

use type2network::FromNetworkOrder;
use type2network_derive::FromNetwork;

use super::domain::DomainName;

#[derive(Debug, Default, FromNetwork)]
pub(super) struct AFSDB<'a> {
    subtype: u16,
    hostname: DomainName<'a>,
}

impl<'a> fmt::Display for AFSDB<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ", self.subtype, self.hostname)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::DNSResult,
        rfc::{afsdb::AFSDB, rdata::RData, response::Response},
        test_rdata,
        tests::{get_pcap_buffer, read_pcap_sample},
    };

    use type2network::FromNetworkOrder;

    test_rdata!(
        "./tests/afsdb.pcap",
        RData::AFSDB,
        (|x: &AFSDB, _| {
            assert_eq!(x.subtype, 1u16);
            assert_eq!(x.hostname.to_string(), "panix.netmeister.org.");
        })
    );
}
