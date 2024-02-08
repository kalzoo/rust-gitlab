// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Delete a runner by authentication token.
#[derive(Debug, Builder, Clone)]
pub struct DeleteRunnerByToken<'a> {
    /// The token of the runner.
    #[builder(setter(into))]
    token: Cow<'a, str>,
}

impl<'a> DeleteRunnerByToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteRunnerByTokenBuilder<'a> {
        DeleteRunnerByTokenBuilder::default()
    }
}

impl<'a> Endpoint for DeleteRunnerByToken<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "runners".into()
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

    use crate::api::runners::{DeleteRunnerByToken, DeleteRunnerByTokenBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn token_is_required() {
        let err = DeleteRunnerByToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRunnerByTokenBuilderError, "token");
    }

    #[test]
    fn token_is_sufficient() {
        DeleteRunnerByToken::builder()
            .token("blah")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str("token=blah")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteRunnerByToken::builder()
            .token("blah")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
