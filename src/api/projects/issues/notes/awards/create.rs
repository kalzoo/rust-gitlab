// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Create a new award on a note on an issue on a project.
#[derive(Debug, Builder, Clone)]
pub struct CreateIssueNoteAward<'a> {
    /// The project the issue belongs to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The issue of the note to add the award to.
    issue: u64,
    /// The ID of the note.
    note: u64,
    /// The award to give to the note (without colons).
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> CreateIssueNoteAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueNoteAwardBuilder<'a> {
        CreateIssueNoteAwardBuilder::default()
    }
}

impl<'a> Endpoint for CreateIssueNoteAward<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/notes/{}/award_emoji",
            self.project, self.issue, self.note,
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

    use crate::api::projects::issues::notes::awards::{
        CreateIssueNoteAward, CreateIssueNoteAwardBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_note_and_name_are_necessary() {
        let err = CreateIssueNoteAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueNoteAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateIssueNoteAward::builder()
            .issue(1)
            .note(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueNoteAwardBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = CreateIssueNoteAward::builder()
            .project(1)
            .note(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueNoteAwardBuilderError, "issue",);
    }

    #[test]
    fn note_is_necessary() {
        let err = CreateIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .name("award")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueNoteAwardBuilderError, "note");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateIssueNoteAwardBuilderError, "name");
    }

    #[test]
    fn project_issue_note_and_name_are_sufficient() {
        CreateIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .note(1)
            .name("award")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/issues/1/notes/2/award_emoji")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateIssueNoteAward::builder()
            .project("simple/project")
            .issue(1)
            .note(2)
            .name("emoji")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
