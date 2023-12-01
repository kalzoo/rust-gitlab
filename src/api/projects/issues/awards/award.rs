// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for award on an issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct IssueAward<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
    /// The ID of the award.
    award: u64,
}

impl<'a> IssueAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueAwardBuilder<'a> {
        IssueAwardBuilder::default()
    }
}

impl<'a> Endpoint for IssueAward<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/award_emoji/{}",
            self.project, self.issue, self.award,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::awards::{IssueAward, IssueAwardBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_and_award_are_necessary() {
        let err = IssueAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = IssueAward::builder().issue(1).award(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = IssueAward::builder()
            .project(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardBuilderError, "issue");
    }

    #[test]
    fn award_is_necessary() {
        let err = IssueAward::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueAwardBuilderError, "award");
    }

    #[test]
    fn project_issue_and_award_are_sufficient() {
        IssueAward::builder()
            .project(1)
            .issue(1)
            .award(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/award_emoji/2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueAward::builder()
            .project("simple/project")
            .issue(1)
            .award(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
