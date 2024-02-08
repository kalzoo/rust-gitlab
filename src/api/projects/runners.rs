// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Project runner-related API endpoints
//!
//! These endpoints are used for querying and modifying CI runners for a project and their
//! resources.

mod runners;

pub use self::runners::ProjectRunners;
pub use self::runners::ProjectRunnersBuilder;
pub use self::runners::ProjectRunnersBuilderError;
