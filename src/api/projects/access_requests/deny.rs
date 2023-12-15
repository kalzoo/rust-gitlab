// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Deny a user access request to a project
#[derive(Debug, Builder, Clone)]
pub struct ProjectAccessRequestsDeny<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The user ID of the access requester
    user_id: u64,
}

impl<'a> ProjectAccessRequestsDeny<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectAccessRequestsDenyBuilder<'a> {
        ProjectAccessRequestsDenyBuilder::default()
    }
}

impl<'a> Endpoint for ProjectAccessRequestsDeny<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_requests/{}", self.project, self.user_id).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::access_requests::{
        ProjectAccessRequestsDeny, ProjectAccessRequestsDenyBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use http::Method;

    #[test]
    fn project_is_needed() {
        let err = ProjectAccessRequestsDeny::builder()
            .user_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ProjectAccessRequestsDenyBuilderError, "project");
    }

    #[test]
    fn user_id_is_needed() {
        let err = ProjectAccessRequestsDeny::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, ProjectAccessRequestsDenyBuilderError, "user_id");
    }

    #[test]
    fn user_project_is_sufficient() {
        ProjectAccessRequestsDeny::builder()
            .project(1)
            .user_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/access_requests/1")
            .method(Method::DELETE)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessRequestsDeny::builder()
            .project("simple/project")
            .user_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
