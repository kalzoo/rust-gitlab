// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Revoke a project access token.
#[derive(Debug, Builder, Clone)]
pub struct RevokeProjectAccessToken<'a> {
    /// The ID of the project.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the token to delete.
    id: u64,
}

impl<'a> RevokeProjectAccessToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RevokeProjectAccessTokenBuilder<'a> {
        RevokeProjectAccessTokenBuilder::default()
    }
}

impl<'a> Endpoint for RevokeProjectAccessToken<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_tokens/{}", self.project, self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::access_tokens::{
        RevokeProjectAccessToken, RevokeProjectAccessTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_id_is_necessary() {
        let err = RevokeProjectAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RevokeProjectAccessTokenBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = RevokeProjectAccessToken::builder()
            .id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RevokeProjectAccessTokenBuilderError, "project");
    }

    #[test]
    fn id_is_necessary() {
        let err = RevokeProjectAccessToken::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, RevokeProjectAccessTokenBuilderError, "id");
    }

    #[test]
    fn project_and_id_are_sufficient() {
        RevokeProjectAccessToken::builder()
            .project(1)
            .id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/1/access_tokens/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RevokeProjectAccessToken::builder()
            .project(1)
            .id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
