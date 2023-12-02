// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Ununarchives the project if the user is either an administrator or the owner of this project.
#[derive(Debug, Builder, Clone)]
pub struct UnarchiveProject<'a> {
    /// The project to unarchive.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> UnarchiveProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnarchiveProjectBuilder<'a> {
        UnarchiveProjectBuilder::default()
    }
}

impl<'a> Endpoint for UnarchiveProject<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/unarchive", self.project).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::{UnarchiveProject, UnarchiveProjectBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = UnarchiveProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnarchiveProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        UnarchiveProject::builder()
            .project("project")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/project%2Fsubproject/unarchive")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnarchiveProject::builder()
            .project("project/subproject")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
