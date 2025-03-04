use std::fmt;

use type2network::FromNetworkOrder;
use type2network_derive::FromNetwork;

use serde::Serialize;

use super::domain::DomainName;

// NS resource record
#[derive(Debug, Default, FromNetwork, Serialize)]
pub struct NS(pub DomainName);

impl fmt::Display for NS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dns::rfc::{rdata::RData, response::Response},
        dns::tests::get_packets,
        test_rdata,
    };

    use type2network::FromNetworkOrder;

    use super::NS;

    test_rdata!(
        rdata,
        "./tests/pcap/ns.pcap",
        false,
        1,
        RData::NS,
        (|x: &NS, _| {
            assert_eq!(&x.to_string(), "panix.netmeister.org.");
        })
    );
}
