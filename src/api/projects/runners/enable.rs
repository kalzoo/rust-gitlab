// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Enable a runner on a project.
#[derive(Debug, Builder, Clone)]
pub struct EnableProjectRunner {
    /// The ID or URL-encoded path of the project.
    project: u64,
    /// The ID of the runner.
    runner: u64,
}

impl EnableProjectRunner {
    /// Create a builder for the endpoint.
    pub fn builder() -> EnableProjectRunnerBuilder {
        EnableProjectRunnerBuilder::default()
    }
}

impl Endpoint for EnableProjectRunner {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/runners", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("runner_id", self.runner);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::runners::{EnableProjectRunner, EnableProjectRunnerBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_runner_are_required() {
        let err = EnableProjectRunner::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EnableProjectRunnerBuilderError, "project");
    }

    #[test]
    fn project_is_required() {
        let err = EnableProjectRunner::builder()
            .runner(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EnableProjectRunnerBuilderError, "project");
    }

    #[test]
    fn runner_is_required() {
        let err = EnableProjectRunner::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EnableProjectRunnerBuilderError, "runner");
    }

    #[test]
    fn project_and_runner_are_sufficient() {
        EnableProjectRunner::builder()
            .project(1)
            .runner(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1/runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str("runner_id=2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EnableProjectRunner::builder()
            .project(1)
            .runner(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
