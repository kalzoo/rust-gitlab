// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Global deploy key related API endpoints
//!
//! These endpoints are used for querying deploy keys across the entire instance.

mod job;

pub use self::job::Job;
pub use self::job::JobBuilder;
pub use self::job::JobBuilderError;
