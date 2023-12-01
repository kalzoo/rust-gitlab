// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project issue award API endpoints.
//!
//! These endpoints are used for querying project issue awards.

mod award;
mod awards;
mod create;
mod delete;

pub use self::award::IssueAward;
pub use self::award::IssueAwardBuilder;
pub use self::award::IssueAwardBuilderError;

pub use self::awards::IssueAwards;
pub use self::awards::IssueAwardsBuilder;
pub use self::awards::IssueAwardsBuilderError;

pub use self::create::CreateIssueAward;
pub use self::create::CreateIssueAwardBuilder;
pub use self::create::CreateIssueAwardBuilderError;

pub use self::delete::DeleteIssueAward;
pub use self::delete::DeleteIssueAwardBuilder;
pub use self::delete::DeleteIssueAwardBuilderError;
