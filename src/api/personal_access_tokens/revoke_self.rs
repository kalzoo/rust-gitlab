// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Revoke a personal access token.
#[derive(Debug, Builder, Clone)]
pub struct RevokePersonalAccessTokenSelf {}

impl RevokePersonalAccessTokenSelf {
    /// Create a builder for the endpoint.
    pub fn builder() -> RevokePersonalAccessTokenSelfBuilder {
        RevokePersonalAccessTokenSelfBuilder::default()
    }
}

impl Endpoint for RevokePersonalAccessTokenSelf {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "personal_access_tokens/self".into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::personal_access_tokens::RevokePersonalAccessTokenSelf;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn default_is_sufficient() {
        RevokePersonalAccessTokenSelf::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("personal_access_tokens/self")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RevokePersonalAccessTokenSelf::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
