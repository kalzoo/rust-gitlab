// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for access requests to a project.
#[derive(Debug, Builder, Clone)]
pub struct ProjectAccessRequests<'a> {
    /// The project to query for pipelines.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> ProjectAccessRequests<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectAccessRequestsBuilder<'a> {
        ProjectAccessRequestsBuilder::default()
    }
}

impl<'a> Endpoint for ProjectAccessRequests<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_requests", self.project).into()
    }
}

impl<'a> Pageable for ProjectAccessRequests<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::access_requests::{
        ProjectAccessRequests, ProjectAccessRequestsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = ProjectAccessRequests::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectAccessRequestsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectAccessRequests::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/access_requests")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessRequests::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
