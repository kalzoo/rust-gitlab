// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Get a single personal access token.
#[derive(Debug, Builder, Clone)]
pub struct PersonalAccessToken {
    /// The ID of the personal access token.
    id: u64,
}

impl PersonalAccessToken {
    /// Create a builder for the endpoint.
    pub fn builder() -> PersonalAccessTokenBuilder {
        PersonalAccessTokenBuilder::default()
    }
}

impl Endpoint for PersonalAccessToken {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("personal_access_tokens/{}", self.id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::personal_access_tokens::{
        PersonalAccessToken, PersonalAccessTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn id_is_necessary() {
        let err = PersonalAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PersonalAccessTokenBuilderError, "id");
    }

    #[test]
    fn id_is_sufficient() {
        PersonalAccessToken::builder().id(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessToken::builder().id(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
