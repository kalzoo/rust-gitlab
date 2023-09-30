// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project push-rule related API endpoints
//!
//! These endpoints are to manage [push rules](https://docs.gitlab.com/ee/api/projects.html#get-project-push-rules)
//! for projects.

mod edit;

pub use edit::EditProjectPushRule;
pub use edit::EditProjectPushRuleBuilder;
pub use edit::EditProjectPushRuleBuilderError;
