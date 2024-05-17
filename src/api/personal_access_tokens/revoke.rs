// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Revoke a personal access token.
#[derive(Debug, Builder, Clone)]
pub struct RevokePersonalAccessToken {
    /// The ID of the token to delete.
    id: u64,
}

impl RevokePersonalAccessToken {
    /// Create a builder for the endpoint.
    pub fn builder() -> RevokePersonalAccessTokenBuilder {
        RevokePersonalAccessTokenBuilder::default()
    }
}

impl Endpoint for RevokePersonalAccessToken {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("personal_access_tokens/{}", self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::personal_access_tokens::{
        RevokePersonalAccessToken, RevokePersonalAccessTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn id_is_necessary() {
        let err = RevokePersonalAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RevokePersonalAccessTokenBuilderError, "id");
    }

    #[test]
    fn id_is_sufficient() {
        RevokePersonalAccessToken::builder().id(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("personal_access_tokens/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RevokePersonalAccessToken::builder().id(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
