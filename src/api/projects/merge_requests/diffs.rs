// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those s.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Shows information of a merge request including its files and changes.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MergeRequestDiffs<'a> {
    /// The project with the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,

    /// Return diffs as unified diffs.
    #[builder(default)]
    unidiff: Option<bool>,
}

impl<'a> MergeRequestDiffs<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestDiffsBuilder<'a> {
        MergeRequestDiffsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestDiffs<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/diffs",
            self.project, self.merge_request,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("unidiff", self.unidiff);

        params
    }
}

impl<'a> Pageable for MergeRequestDiffs<'a> {}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::{MergeRequestDiffs, MergeRequestDiffsBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequestDiffs::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestDiffsBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestDiffs::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestDiffsBuilderError, "project");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequestDiffs::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, MergeRequestDiffsBuilderError, "merge_request");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestDiffs::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/merge_requests/1/diffs")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestDiffs::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unidiff() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/merge_requests/1/diffs")
            .add_query_params(&[("unidiff", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestDiffs::builder()
            .project("simple/project")
            .merge_request(1)
            .unidiff(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
