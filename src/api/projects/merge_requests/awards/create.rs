// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new award on an merge_request on a project.
#[derive(Debug, Builder, Clone)]
pub struct CreateMergeRequestAward<'a> {
    /// The project the merge_request belongs to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The merge_request to add the award to.
    merge_request: u64,
    /// The award to give to the merge_request (without colons).
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> CreateMergeRequestAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateMergeRequestAwardBuilder<'a> {
        CreateMergeRequestAwardBuilder::default()
    }
}

impl<'a> Endpoint for CreateMergeRequestAward<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/award_emoji",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("name", self.name.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::merge_requests::awards::{
        CreateMergeRequestAward, CreateMergeRequestAwardBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_merge_request_and_name_are_necessary() {
        let err = CreateMergeRequestAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateMergeRequestAward::builder()
            .merge_request(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestAwardBuilderError, "project");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = CreateMergeRequestAward::builder()
            .project(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreateMergeRequestAwardBuilderError,
            "merge_request",
        );
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateMergeRequestAward::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateMergeRequestAwardBuilderError, "name");
    }

    #[test]
    fn project_merge_request_and_name_are_sufficient() {
        CreateMergeRequestAward::builder()
            .project(1)
            .merge_request(1)
            .name("award")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/award_emoji")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestAward::builder()
            .project("simple/project")
            .merge_request(1)
            .name("emoji")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
