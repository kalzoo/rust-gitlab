// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Get access tokens of a project.
#[derive(Debug, Builder, Clone)]
pub struct ProjectAccessTokens<'a> {
    /// The project for which to list tokens.
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> ProjectAccessTokens<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectAccessTokensBuilder<'a> {
        ProjectAccessTokensBuilder::default()
    }
}

impl<'a> Endpoint for ProjectAccessTokens<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_tokens", self.project).into()
    }
}

impl<'a> Pageable for ProjectAccessTokens<'a> {}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::access_tokens::{
        ProjectAccessTokens, ProjectAccessTokensBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_required() {
        let err = ProjectAccessTokens::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectAccessTokensBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectAccessTokens::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/1/access_tokens")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessTokens::builder().project(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
