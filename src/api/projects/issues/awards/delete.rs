// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete an existing award on an issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteIssueAward<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
    /// The ID of the award.
    award: u64,
}

impl<'a> DeleteIssueAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteIssueAwardBuilder<'a> {
        DeleteIssueAwardBuilder::default()
    }
}

impl<'a> Endpoint for DeleteIssueAward<'a> {
    fn method(&self) -> Method {
        Method::DELETE
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
    use http::Method;

    use crate::api::projects::issues::awards::{DeleteIssueAward, DeleteIssueAwardBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_and_award_are_necessary() {
        let err = DeleteIssueAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteIssueAward::builder()
            .issue(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueAwardBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = DeleteIssueAward::builder()
            .project(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueAwardBuilderError, "issue");
    }

    #[test]
    fn award_is_necessary() {
        let err = DeleteIssueAward::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueAwardBuilderError, "award");
    }

    #[test]
    fn project_issue_and_award_are_sufficient() {
        DeleteIssueAward::builder()
            .project(1)
            .issue(1)
            .award(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/issues/1/award_emoji/2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteIssueAward::builder()
            .project("simple/project")
            .issue(1)
            .award(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
