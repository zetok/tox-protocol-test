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

//! https://toktok.github.io/spec#test-k-bucket-index

use tox::toxcore::binary_io::*;
use tox::toxcore::crypto_core::*;
use tox::toxcore::dht::*;

use super::super::result::*;


/** Function to parse KBucketIndex and calculate index from PKs.

    Returns `0` byte if supplied PKs are qual, or `1` byte followed by index
    byte otherwise.
*/
pub fn parse_kbucket_index(bytes: &[u8]) -> Vec<u8> {
    let pk1 = match PublicKey::from_slice(&bytes[..PUBLICKEYBYTES]) {
        Some(pk) => pk,
        None => return Failure::from_str("Wrong amount of bytes for PK1!").to_bytes(),
    };

    let pk2 = match PublicKey::from_slice(&bytes[PUBLICKEYBYTES..(2 * PUBLICKEYBYTES)]) {
        Some(pk) => pk,
        None => return Failure::from_str("Wrong amount of bytes for PK2!").to_bytes(),
    };

    match kbucket_index(&pk1, &pk2) {
        Some(i) => vec![1, i],
        None    => vec![0],
    }
}
