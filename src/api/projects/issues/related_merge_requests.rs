// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for merge requests related to an issue
#[derive(Debug, Builder, Clone)]
pub struct RelatedMergeRequests<'a> {
    /// The project to of the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue
    issue: u64,
}

impl<'a> RelatedMergeRequests<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RelatedMergeRequestsBuilder<'a> {
        RelatedMergeRequestsBuilder::default()
    }
}

impl<'a> Endpoint for RelatedMergeRequests<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/related_merge_requests",
            self.project, self.issue,
        )
        .into()
    }
}

impl<'a> Pageable for RelatedMergeRequests<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::{RelatedMergeRequests, RelatedMergeRequestsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_issue_are_needed() {
        let err = RelatedMergeRequests::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RelatedMergeRequestsBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = RelatedMergeRequests::builder()
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RelatedMergeRequestsBuilderError, "project");
    }

    #[test]
    fn issue_is_needed() {
        let err = RelatedMergeRequests::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RelatedMergeRequestsBuilderError, "issue");
    }

    #[test]
    fn project_and_issue_are_sufficient() {
        RelatedMergeRequests::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/related_merge_requests")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RelatedMergeRequests::builder()
            .project("simple/project")
            .issue(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
