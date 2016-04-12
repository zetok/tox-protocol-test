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

//! Returning result for tests.

use tox::toxcore::binary_io::*;

/** Struct for returning "Failure" data to hstox.

    Serialized format:

    Field | Length
    ------|------
    0x00 (`Failure`) | 1 byte
    lenght | 8 bytes
    error message | `$length` bytes
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Failure {
    err_msg: String,
}

impl Failure {
    /// Create a new `Failure` with an empty error.
    pub fn new() -> Self {
        Failure { err_msg: String::new() }
    }

    /// Create a new `Failure` from a `&str`.
    pub fn from_str(s: &str) -> Self {
        Failure { err_msg: s.to_string() }
    }
}

impl ToBytes for Failure {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.err_msg.len() + 9);
        result.push(0x00); // Faliure tag

        let msg_len: u64 = (self.err_msg.len() as u64).to_be();
        result.extend_from_slice(&u64_to_array(msg_len)); // lenght of msg

        result.extend_from_slice(self.err_msg.as_bytes());
        result
    }
}


/** Struct for returning `Success` data to hstox.

    Serialized format:

    Field | Length
    ------|------
    0x01 (`Success`) | 1 byte
    result data | depends on the name
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Success {
    res: Vec<u8>,
}

impl Success {
    pub fn new(bytes: &[u8]) -> Self {
        Success { res: bytes.to_vec() }
    }
}

impl ToBytes for Success {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.res.len() + 1);
        result.push(0x01); // Success tag

        result.extend_from_slice(&self.res);
        result
    }
}


/** Struct for returning `Skipped` data to hstox.

    Serialized format:

    Field | Length
    ------|------
    0x02 (`Skipped`) | 1 byte
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Skipped;

impl Skipped {
    /// Create new `Skipped`.
    pub fn new() -> Self {
        Skipped
    }
}

impl ToBytes for Skipped {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x02]
    }
}
