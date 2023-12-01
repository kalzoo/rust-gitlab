// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete an existing award on a merge_request within a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteMergeRequestAward<'a> {
    /// The project to query for the merge_request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge_request.
    merge_request: u64,
    /// The ID of the award.
    award: u64,
}

impl<'a> DeleteMergeRequestAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteMergeRequestAwardBuilder<'a> {
        DeleteMergeRequestAwardBuilder::default()
    }
}

impl<'a> Endpoint for DeleteMergeRequestAward<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/award_emoji/{}",
            self.project, self.merge_request, self.award,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::awards::{
        DeleteMergeRequestAward, DeleteMergeRequestAwardBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_merge_request_and_award_are_necessary() {
        let err = DeleteMergeRequestAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteMergeRequestAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteMergeRequestAward::builder()
            .merge_request(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteMergeRequestAwardBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = DeleteMergeRequestAward::builder()
            .project(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            DeleteMergeRequestAwardBuilderError,
            "merge_request"
        );
    }

    #[test]
    fn award_is_necessary() {
        let err = DeleteMergeRequestAward::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteMergeRequestAwardBuilderError, "award");
    }

    #[test]
    fn project_merge_request_and_award_are_sufficient() {
        DeleteMergeRequestAward::builder()
            .project(1)
            .merge_request(1)
            .award(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/merge_requests/1/award_emoji/2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteMergeRequestAward::builder()
            .project("simple/project")
            .merge_request(1)
            .award(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
