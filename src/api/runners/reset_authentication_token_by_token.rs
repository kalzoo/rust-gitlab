// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Reset a runner's authentication token by its current token.
#[derive(Debug, Builder, Clone)]
pub struct ResetRunnerAuthenticationTokenByToken<'a> {
    /// The authentication token of the runner.
    #[builder(setter(into))]
    token: Cow<'a, str>,
}

impl<'a> ResetRunnerAuthenticationTokenByToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ResetRunnerAuthenticationTokenByTokenBuilder<'a> {
        ResetRunnerAuthenticationTokenByTokenBuilder::default()
    }
}

impl<'a> Endpoint for ResetRunnerAuthenticationTokenByToken<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "runners/reset_authentication_token".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("token", self.token.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{
        ResetRunnerAuthenticationTokenByToken, ResetRunnerAuthenticationTokenByTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn token_is_required() {
        let err = ResetRunnerAuthenticationTokenByToken::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            ResetRunnerAuthenticationTokenByTokenBuilderError,
            "token"
        );
    }

    #[test]
    fn token_is_sufficient() {
        ResetRunnerAuthenticationTokenByToken::builder()
            .token("blah")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners/reset_authentication_token")
            .content_type("application/x-www-form-urlencoded")
            .body_str("token=blah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ResetRunnerAuthenticationTokenByToken::builder()
            .token("blah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
