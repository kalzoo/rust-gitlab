// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Reset a runner's authentication token.
#[derive(Debug, Builder, Clone)]
pub struct ResetRunnerAuthenticationToken {
    /// The ID of the runner.
    runner: u64,
}

impl ResetRunnerAuthenticationToken {
    /// Create a builder for the endpoint.
    pub fn builder() -> ResetRunnerAuthenticationTokenBuilder {
        ResetRunnerAuthenticationTokenBuilder::default()
    }
}

impl Endpoint for ResetRunnerAuthenticationToken {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("runners/{}/reset_authentication_token", self.runner).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{
        ResetRunnerAuthenticationToken, ResetRunnerAuthenticationTokenBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn runner_is_required() {
        let err = ResetRunnerAuthenticationToken::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            ResetRunnerAuthenticationTokenBuilderError,
            "runner"
        );
    }

    #[test]
    fn runner_is_sufficient() {
        ResetRunnerAuthenticationToken::builder()
            .runner(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners/1/reset_authentication_token")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ResetRunnerAuthenticationToken::builder()
            .runner(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
