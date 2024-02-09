// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::{Builder, UninitializedFieldError};

use crate::api::endpoint_prelude::*;

/// Verify a runner.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct VerifyRunner<'a> {
    /// The token of the runner.
    #[builder(setter(into))]
    token: Cow<'a, str>,

    /// The runner's system ID.
    ///
    /// Required if `token` starts with `glrt-`.
    #[builder(setter(into), default)]
    system_id: Option<Cow<'a, str>>,
}

impl<'a> VerifyRunner<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VerifyRunnerBuilder<'a> {
        VerifyRunnerBuilder::default()
    }
}

impl<'a> VerifyRunnerBuilder<'a> {
    fn validate(&self) -> Result<(), VerifyRunnerBuilderError> {
        if let Some(ref token) = self.token {
            if token.starts_with("glrt-") && self.system_id.is_none() {
                return Err(UninitializedFieldError::new("system_id").into());
            }
        }

        Ok(())
    }
}

impl<'a> Endpoint for VerifyRunner<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "runners/verify".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("token", self.token.as_ref())
            .push_opt("system_id", self.system_id.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{VerifyRunner, VerifyRunnerBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn token_is_required() {
        let err = VerifyRunner::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, VerifyRunnerBuilderError, "token");
    }

    #[test]
    fn token_is_sufficient() {
        VerifyRunner::builder().token("blah").build().unwrap();
    }

    #[test]
    fn token_is_required_for_glrt_prefix() {
        let err = VerifyRunner::builder()
            .token("glrt-deadbeef")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, VerifyRunnerBuilderError, "system_id");
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners/verify")
            .content_type("application/x-www-form-urlencoded")
            .body_str("token=blah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = VerifyRunner::builder().token("blah").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
