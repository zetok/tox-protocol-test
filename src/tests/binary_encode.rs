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

/*! Parsing tests for binary encoding.

    https://toktok.github.io/spec#test-binary-encode
*/


use tox::toxcore::binary_io::*;

use super::super::deconstructed::*;
use super::super::result::*;


/** Function to parse binary encoding of PackedNode (aka NodeInfo) from a
    deconstructed value.
*/
pub fn parse_encode_packed_node(bytes: &[u8]) -> Vec<u8> {
    match DecPackedNode::from_bytes(bytes) {
        Some(dpn) => Success::new(&dpn.as_packed_node().to_bytes()).to_bytes(),
        None      => Failure::new().to_bytes(),
    }
}
