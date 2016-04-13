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

//! https://toktok.github.io/spec#test-nonce-increment

use tox::toxcore::binary_io::*;
use tox::toxcore::crypto_core::*;

use super::super::result::*;

/** Function to parse `Nonce Increment` bytes into `Nonce`, and increment it.

    Can return `Failure` if bytes can't be parsed as `Nonce`.
*/
pub fn parse_nonce(bytes: &[u8]) -> Vec<u8> {
    let mut nonce = match Nonce::from_slice(&bytes[..NONCEBYTES]) {
        Some(n) => n,
        None => return Failure::from_str("Wrong amount of bytes for nonce!").to_bytes(),
    };

    increment_nonce(&mut nonce);

    let Nonce(n) = nonce;
    Success::new(&n).to_bytes()
}
