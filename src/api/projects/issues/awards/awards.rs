// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for awards on an issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct IssueAwards<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
}

impl<'a> IssueAwards<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueAwardsBuilder<'a> {
        IssueAwardsBuilder::default()
    }
}

impl<'a> Endpoint for IssueAwards<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/award_emoji",
            self.project, self.issue,
        )
        .into()
    }
}

impl<'a> Pageable for IssueAwards<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::awards::{IssueAwards, IssueAwardsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_necessary() {
        let err = IssueAwards::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = IssueAwards::builder().issue(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardsBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = IssueAwards::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardsBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        IssueAwards::builder().project(1).issue(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/award_emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueAwards::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
