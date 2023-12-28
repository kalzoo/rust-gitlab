// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Submit an access request to a project for the clients user token.
#[derive(Debug, Builder, Clone)]
pub struct ProjectAccessRequest<'a> {
    /// The project to query for pipelines.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> ProjectAccessRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectAccessRequestBuilder<'a> {
        ProjectAccessRequestBuilder::default()
    }
}

impl<'a> Endpoint for ProjectAccessRequest<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_requests", self.project).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::access_requests::{
        ProjectAccessRequest, ProjectAccessRequestBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use http::Method;

    #[test]
    fn project_is_needed() {
        let err = ProjectAccessRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectAccessRequestBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectAccessRequest::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/access_requests")
            .method(Method::POST)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessRequest::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
