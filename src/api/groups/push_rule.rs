// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Group push-rule related API endpoints
//!
//! These endpoints are to manage [push rules](https://docs.gitlab.com/ee/api/groups.html#get-group-push-rules)
//! for groups.

mod edit;

pub use edit::EditGroupPushRule;
pub use edit::EditGroupPushRuleBuilder;
pub use edit::EditGroupPushRuleBuilderError;
