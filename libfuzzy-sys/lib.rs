// ssdeep-rs: A Rust wrapper for ssdeep.
//
// Copyright (c) 2016 Petr Zemek <s3rvac@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::c_uchar;
use libc::uint32_t;

// From fuzzy.h:

// Length of an individual fuzzy hash signature component.
const SPAMSUM_LENGTH: usize = 64;

// The longest possible length for a fuzzy hash signature.
pub const FUZZY_MAX_RESULT: usize = 2 * SPAMSUM_LENGTH + 20;

extern "C" {
    // int fuzzy_compare(const char *sig1, const char *sig2);
    pub fn fuzzy_compare(sig1: *const c_char, sig2: *const c_char) -> c_int;

    // int fuzzy_hash_buf(const unsigned char *buf, uint32_t buf_len, char *result);
    pub fn fuzzy_hash_buf(buf: *const c_uchar, buf_len: uint32_t, result: *mut c_char) -> c_int;

    // int fuzzy_hash_filename(const char *filename, char *result);
    pub fn fuzzy_hash_filename(filename: *const c_char, result: *mut c_char) -> c_int;
}