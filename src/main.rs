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

//! https://toktok.github.io/spec#test-protocol


use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::string::String;
use std::u64;

extern crate tox;
use tox::toxcore::binary_io::*;

mod result;
use result::*;

mod tests;
use tests::*;


// All sent to hstox numbers are in Big Endian.

/// Write debug message to file
#[allow(dead_code)]
fn debug(msg: &str) {
    let mut w = OpenOptions::new().write(true).create(true).truncate(true)
                    .open("./debug.txt").unwrap();
    drop(w.write_all(&msg.as_bytes()));
}

/// Position where test name starts.
const NAME_POS: usize = 8;

/// Parse test and return resulting bytes.
fn parse(bytes: &[u8]) -> Vec<u8> {

    let test_name_len = {
        let num = array_to_u64(&[bytes[0], bytes[1], bytes[2], bytes[3],
                                 bytes[4], bytes[5], bytes[6], bytes[7]]);
        u64::from_be(num) as usize
    };

    // starting position of actual bytes of data
    let b_to_parse = NAME_POS + test_name_len;

    match String::from_utf8(bytes[NAME_POS..b_to_parse].to_vec()) {
        Ok(ref s) if s == "TestFailure" => Failure::new().to_bytes(),
        Ok(ref s) if s == "TestSuccess" => Success::new(&[]).to_bytes(),
        Ok(ref s) if s == "SkippedTest" => Skipped::new().to_bytes(),
        Ok(ref s) if s == "Distance" =>
            parse_distance(&bytes[b_to_parse..]),
        Ok(ref s) if s == "NonceIncrement" =>
            parse_nonce(&bytes[b_to_parse..]),
        Ok(ref s) if s == "KBucektIndex" =>
            parse_kbucket_index(&bytes[b_to_parse..]),
        _ => Skipped::new().to_bytes(), // skip everything else
    }
}


fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    // according to iphy 10KB should be enough
    let mut buf = [0; 10240];
    debug("about to get bytes");
    match stdin.read(&mut buf) {
        Ok(num) => {
            debug(&format!("got bytes: {}", num));
            drop(stdout.write(&parse(&buf[..num])));
        },
        Err(e)  => debug(&format!("ain't got no bytes: {}", e)),
    }
}
