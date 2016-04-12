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

//! https://toktok.github.io/spec#test-distance

use std::cmp::Ordering;

use tox::toxcore::binary_io::*;
use tox::toxcore::crypto_core::*;
use tox::toxcore::dht::*;

use super::super::result::*;


/** Function to parse bytes as PKs and compute relative distance between them.

    Returned bytes are either `Success` with encoded into bytes relative
    distance indicator, or a `Failure` bytes with an error message.
*/
pub fn parse_distance(bytes: &[u8]) -> Vec<u8> {
    let own_pk = match PublicKey::from_slice(&bytes[..PUBLICKEYBYTES]) {
        Some(pk) => pk,
        None => return Failure::from_str("Failed to parse bytes into \"own\" PK.").to_bytes(),
    };

    let alice_pk = match PublicKey::from_slice(&bytes[PUBLICKEYBYTES..(2 * PUBLICKEYBYTES)]) {
        Some(pk) => pk,
        None => return Failure::from_str("Failed to parse bytes into Alice PK.").to_bytes(),
    };

    let bob_pk = match PublicKey::from_slice(&bytes[(2 * PUBLICKEYBYTES)..(3 * PUBLICKEYBYTES)]) {
        Some(pk) => pk,
        None => return Failure::from_str("Failed to parse bytes into Bob PK.").to_bytes(),
    };

    match own_pk.distance(&alice_pk, &bob_pk) {
        Ordering::Less =>    Success::new(&[0]).to_bytes(),
        Ordering::Equal =>   Success::new(&[1]).to_bytes(),
        Ordering::Greater => Success::new(&[2]).to_bytes(),
    }
}
