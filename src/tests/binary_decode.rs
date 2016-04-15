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

/*! Parsing tests for binary decoding.

    https://toktok.github.io/spec#test-binary-decode
*/


use tox::toxcore::binary_io::*;
use tox::toxcore::dht::PackedNode;

use super::super::deconstructed::*;
use super::super::result::*;


/** Function to parse `bytes → PackedNode → DecPackedNode → bytes`.

    `PackedNode` bytes are prepended with length of bytes to parse.

    https://toktok.github.io/spec#test-binary-decode
*/
pub fn parse_decode_packed_node(bytes: &[u8]) -> Vec<u8> {
    let to_parse = u64::from_be(array_to_u64(&[bytes[0], bytes[1], bytes[2],
            bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) as usize + 8;
    match PackedNode::from_bytes(&bytes[8..to_parse]) {
    //match PackedNode::from_bytes(bytes) {
        Some(pn) => Success::new(&DecPackedNode::from_packed_node(&pn).to_bytes()).to_bytes(),
        None => Failure::from_str("Failed to parse as PackedNode").to_bytes(),
    }
}
