// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Deletes the project if the user is either an administrator or the owner of this project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteProject<'a> {
    /// The project to delete.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> DeleteProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteProjectBuilder<'a> {
        DeleteProjectBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProject<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}", self.project).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::{DeleteProject, DeleteProjectBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = DeleteProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        DeleteProject::builder().project("project").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/1337")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteProject::builder().project(1337).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
