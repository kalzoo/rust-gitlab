// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new award on an issue on a project.
#[derive(Debug, Builder, Clone)]
pub struct CreateIssueAward<'a> {
    /// The project the issue belongs to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The issue to add the award to.
    issue: u64,
    /// The award to give to the issue (without colons).
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> CreateIssueAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueAwardBuilder<'a> {
        CreateIssueAwardBuilder::default()
    }
}

impl<'a> Endpoint for CreateIssueAward<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/award_emoji",
            self.project, self.issue,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("name", self.name.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::issues::awards::{CreateIssueAward, CreateIssueAwardBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_and_name_are_necessary() {
        let err = CreateIssueAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateIssueAward::builder()
            .issue(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueAwardBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = CreateIssueAward::builder()
            .project(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueAwardBuilderError, "issue",);
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateIssueAward::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueAwardBuilderError, "name");
    }

    #[test]
    fn project_issue_and_name_are_sufficient() {
        CreateIssueAward::builder()
            .project(1)
            .issue(1)
            .name("award")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues/1/award_emoji")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssueAward::builder()
            .project("simple/project")
            .issue(1)
            .name("emoji")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
