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
pub struct RotatePersonalAccessTokenSelf {
    /// The new expiration for the token.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl RotatePersonalAccessTokenSelf {
    /// Create a builder for the endpoint.
    pub fn builder() -> RotatePersonalAccessTokenSelfBuilder {
        RotatePersonalAccessTokenSelfBuilder::default()
    }
}

impl Endpoint for RotatePersonalAccessTokenSelf {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "personal_access_tokens/self/rotate".into()
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

    use crate::api::personal_access_tokens::RotatePersonalAccessTokenSelf;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn default_is_sufficient() {
        RotatePersonalAccessTokenSelf::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("personal_access_tokens/self/rotate")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RotatePersonalAccessTokenSelf::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("personal_access_tokens/self/rotate")
            .content_type("application/x-www-form-urlencoded")
            .body_str("expires_at=2024-06-01")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RotatePersonalAccessTokenSelf::builder()
            .expires_at(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
