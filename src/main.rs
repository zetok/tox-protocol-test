/*
    Copyright © 2016 Zetok Zalbavar <zexavexxe@gmail.com>

    This program is libre software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/


extern crate tox;

use std::u64;
use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;

use std::string::String;

use tox::toxcore::dht::*;
use tox::toxcore::binary_io::*;


// All sent to hstox numbers are in Big Endian.

/// Write debug message to file
#[allow(dead_code)]
fn debug(msg: &str) {
    let mut w = OpenOptions::new().append(true).create(true)
                    .open("./debug.txt").unwrap();
    drop(w.write_all(&msg.as_bytes()));
}

/// Struct for returning "Failure" data to hstox.
///
/// Serialized format:
///
/// Field | Length
/// ------|------
/// 0x00 (`Failure`) | 1 byte
/// lenght | 8 bytes
/// error message | `$length` bytes
struct Failure {
    err_msg: String,
}

impl Failure {
    /// Create a new `Failure` with an empty error.
    fn new() -> Self {
        Failure { err_msg: String::new() }
    }

    /// Create a new `Failure` from a `&str`.
    fn from_str(s: &str) -> Self {
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


/// Struct for returning `Success` data to hstox.
///
/// Serialized format:
///
/// Field | Length
/// ------|------
/// 0x01 (`Success`) | 1 byte
/// result data | depends on the name
struct Success {
    res: Vec<u8>,
}

impl Success {
    fn new(bytes: &[u8]) -> Self {
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


/// Struct for returning `Skipped` data to hstox.
///
/// Serialized format:
///
/// Field | Length
/// ------|------
/// 0x02 (`Skipped`) | 1 byte
struct Skipped;

impl Skipped {
    /// Create new `Skipped`.
    fn new() -> Self {
        Skipped
    }
}


impl ToBytes for Skipped {
    fn to_bytes(&self) -> Vec<u8> {
        vec![0x02]
    }
}


/// Position where test name starts.
const NAME_POS: usize = 8;


/// Function to parse bytes as PackedNode aka NodeInfo and return bytes.
///
/// Returned bytes are either `Success` with encoded into bytes node, or a
/// `Faliure` bytes that contain an error message.
fn parse_node_info(bytes: &[u8]) -> Vec<u8> {
    match PackedNode::from_bytes(bytes) {
        Some(pn) => Success::new(&pn.to_bytes()).to_bytes(),
        None => Failure::from_str("Failed to decode PackedNode.").to_bytes(),
    }
}

/// Parse test and return resulting bytes.
fn parse(bytes: &[u8]) -> Vec<u8> {

    let test_name_len = {
        let num = array_to_u64(&[bytes[0], bytes[1], bytes[2], bytes[3],
                                 bytes[4], bytes[5], bytes[6], bytes[7]]);
        u64::from_be(num) as usize
    };

    match String::from_utf8(bytes[NAME_POS..(NAME_POS + test_name_len)].to_vec()) {
        Ok(ref s) if s == "TestFailure" => Failure::new().to_bytes(),
        Ok(ref s) if s == "TestSuccess" => Success::new(&[]).to_bytes(),
        Ok(ref s) if s == "SkippedTest" => Skipped::new().to_bytes(),
        Ok(ref s) if s == "BinaryDecode NodeInfo" => {
            parse_node_info(&bytes[(NAME_POS + test_name_len + 8)..])
        },
        // for now skip ↓
        Ok(ref s) if s == "Distance" => Skipped::new().to_bytes(), 
        _ => Skipped::new().to_bytes(), // skip everything else
    }
}


fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).unwrap();

    drop(stdout.write(&parse(&buf)));
}