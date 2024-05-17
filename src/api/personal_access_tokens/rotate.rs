// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Rotate a personal access token.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RotatePersonalAccessToken {
    /// The ID of the token to rotate.
    id: u64,
    /// The new expiration for the token.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl RotatePersonalAccessToken {
    /// Create a builder for the endpoint.
    pub fn builder() -> RotatePersonalAccessTokenBuilder {
        RotatePersonalAccessTokenBuilder::default()
    }
}

impl Endpoint for RotatePersonalAccessToken {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("personal_access_tokens/{}/rotate", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt("expires_at", self.expires_at);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::personal_access_tokens::{
        RotatePersonalAccessToken, RotatePersonalAccessTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn id_is_necessary() {
        let err = RotatePersonalAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RotatePersonalAccessTokenBuilderError, "id");
    }

    #[test]
    fn id_is_sufficient() {
        RotatePersonalAccessToken::builder().id(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("personal_access_tokens/1/rotate")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RotatePersonalAccessToken::builder().id(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("personal_access_tokens/1/rotate")
            .content_type("application/x-www-form-urlencoded")
            .body_str("expires_at=2024-06-01")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RotatePersonalAccessToken::builder()
            .id(1)
            .expires_at(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
