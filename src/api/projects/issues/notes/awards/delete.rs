// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete an existing award on a note within an issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteIssueNoteAward<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
    /// The ID of the note.
    note: u64,
    /// The ID of the award.
    award: u64,
}

impl<'a> DeleteIssueNoteAward<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteIssueNoteAwardBuilder<'a> {
        DeleteIssueNoteAwardBuilder::default()
    }
}

impl<'a> Endpoint for DeleteIssueNoteAward<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/notes/{}/award_emoji/{}",
            self.project, self.issue, self.note, self.award,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::issues::notes::awards::{
        DeleteIssueNoteAward, DeleteIssueNoteAwardBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_note_and_award_are_necessary() {
        let err = DeleteIssueNoteAward::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueNoteAwardBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteIssueNoteAward::builder()
            .issue(1)
            .note(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueNoteAwardBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = DeleteIssueNoteAward::builder()
            .project(1)
            .note(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueNoteAwardBuilderError, "issue");
    }

    #[test]
    fn note_is_necessary() {
        let err = DeleteIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .award(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueNoteAwardBuilderError, "note");
    }

    #[test]
    fn award_is_necessary() {
        let err = DeleteIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteIssueNoteAwardBuilderError, "award");
    }

    #[test]
    fn project_issue_note_and_award_are_sufficient() {
        DeleteIssueNoteAward::builder()
            .project(1)
            .issue(1)
            .note(1)
            .award(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/issues/1/notes/2/award_emoji/3")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteIssueNoteAward::builder()
            .project("simple/project")
            .issue(1)
            .note(2)
            .award(3)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
