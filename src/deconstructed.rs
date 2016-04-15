/*
    Copyright © 2016 Zetok Zalbavar <zexavexxe@gmail.com>

    This file is part of Tox.

    Tox is libre software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Tox is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Tox.  If not, see <http://www.gnu.org/licenses/>.
*/

/*! Deconstructed structures.

    https://toktok.github.io/spec#deconstructed-values
*/

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use tox::toxcore::binary_io::*;
use tox::toxcore::crypto_core::*;
use tox::toxcore::dht::*;


/** Deconstructed PackedNode.

    `DecPackedNode` is an equivalent of information provided by the byte
    representation provided by the spec, with part of it conveyed by data
    structures used, rather than explicitly provided, like byte representation
    does.

    Differences in representation, if any, stem from that.

    https://toktok.github.io/spec#deconstructed-node-info
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecPackedNode {
    udp: bool, // the opposite to the value from spec, equivalent to value used
               // by zetox for PackedNode creation
    addr: IpAddr,
    port: u16,
    public_key: PublicKey,
}

impl DecPackedNode {
    pub fn as_packed_node(&self) -> PackedNode {
        PackedNode::new(
            self.udp,
            SocketAddr::new(self.addr, self.port),
            &self.public_key
        )
    }

    pub fn from_packed_node(pn: &PackedNode) -> Self {
        let udp = match pn.ip_type {
            IpType::U4 | IpType::U6 => true,
            _ => false,
        };

        DecPackedNode {
            udp: udp,
            addr: pn.saddr.ip(),
            port: pn.saddr.port(),
            public_key: pn.pk,
        }
    }
}

/// Minimal size in bytes of serialized `DecPackedNode`.
const DEC_PACKED_NODE_MIN: usize = 40;

/// Maximum size in bytes of serialized `DecPackedNode`.
const DEC_PACKED_NODE_MAX: usize = DEC_PACKED_NODE_MIN + 12;

impl ToBytes for DecPackedNode {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(DEC_PACKED_NODE_MIN);
        if self.udp { result.push(0); } else { result.push(1); }
        match self.addr {
            IpAddr::V4(_) => result.push(0),
            IpAddr::V6(_) => result.push(1),
        }
        result.extend_from_slice(&self.addr.to_bytes());
        result.extend_from_slice(&u16_to_array(self.port.to_be()));

        let PublicKey(pk) = self.public_key;
        result.extend_from_slice(&pk);
        result
    }
}

impl FromBytes<DecPackedNode> for DecPackedNode {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < DEC_PACKED_NODE_MIN {
            return None
        }

        // is TCP?
        let mut udp = true;
        if bytes[0] == 1 { udp = false; }

        // is IPv6?
        if bytes[1] == 1 {
            if bytes.len() < DEC_PACKED_NODE_MAX {
                return None
            }

            let ipv6 = Ipv6Addr::from_bytes(&bytes[2..18])
                .expect("Failed to get IPv6 from bytes.");

            let port = u16::from_be(array_to_u16(&[bytes[18], bytes[19]]));
            let pk = PublicKey::from_slice(&bytes[20..(PUBLICKEYBYTES + 20)])
                .expect("Failed to get PK from bytes");

            return Some(DecPackedNode {
                udp: udp,
                addr: IpAddr::V6(ipv6),
                port: port,
                public_key: pk,
            })
        } else { // ← just assume that it's IPv4 if not IPv6
            let ipv4 = Ipv4Addr::new(bytes[2], bytes[3], bytes[4], bytes[5]);
            let port = u16::from_be(array_to_u16(&[bytes[6], bytes[7]]));
            let pk = PublicKey::from_slice(&bytes[8..(PUBLICKEYBYTES + 8)])
                .expect("Failed to get PK from bytes");

            return Some(DecPackedNode {
                udp: udp,
                addr: IpAddr::V4(ipv4),
                port: port,
                public_key: pk,
            })
        }
        None  // :c
    }
}
