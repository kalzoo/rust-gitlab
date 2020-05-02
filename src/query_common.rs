// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Descending
    }
}

impl SortOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc",
        }
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableState {
    Enabled,
    Disabled,
}

impl EnableState {
    pub fn as_str(self) -> &'static str {
        match self {
            EnableState::Enabled => "enabled",
            EnableState::Disabled => "disabled",
        }
    }
}

impl fmt::Display for EnableState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameOrId {
    Name(String),
    Id(u64),
}

const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b'/');

impl fmt::Display for NameOrId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NameOrId::Name(name) => {
                let encoded = utf8_percent_encode(name, PATH_SEGMENT_ENCODE_SET);
                write!(f, "{}", encoded)
            },
            NameOrId::Id(id) => write!(f, "{}", id),
        }
    }
}
