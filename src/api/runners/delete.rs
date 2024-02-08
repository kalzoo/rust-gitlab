// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Delete a runner.
#[derive(Debug, Builder, Clone)]
pub struct DeleteRunner {
    /// The ID of the runner.
    runner: u64,
}

impl DeleteRunner {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteRunnerBuilder {
        DeleteRunnerBuilder::default()
    }
}

impl Endpoint for DeleteRunner {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("runners/{}", self.runner).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{DeleteRunner, DeleteRunnerBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn runner_is_required() {
        let err = DeleteRunner::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteRunnerBuilderError, "runner");
    }

    #[test]
    fn runner_is_sufficient() {
        DeleteRunner::builder().runner(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("runners/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteRunner::builder().runner(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
