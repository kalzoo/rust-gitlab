// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for awards on a note on an issue within a project.
#[derive(Debug, Builder, Clone)]
pub struct IssueNoteAwards<'a> {
    /// The project to query for the issue.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the issue.
    issue: u64,
    /// The ID of the note.
    note: u64,
}

impl<'a> IssueNoteAwards<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueNoteAwardsBuilder<'a> {
        IssueNoteAwardsBuilder::default()
    }
}

impl<'a> Endpoint for IssueNoteAwards<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/issues/{}/notes/{}/award_emoji",
            self.project, self.issue, self.note,
        )
        .into()
    }
}

impl<'a> Pageable for IssueNoteAwards<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::issues::notes::awards::{
        IssueNoteAwards, IssueNoteAwardsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_issue_and_note_are_necessary() {
        let err = IssueNoteAwards::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, IssueNoteAwardsBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = IssueNoteAwards::builder()
            .issue(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueNoteAwardsBuilderError, "project");
    }

    #[test]
    fn issue_is_necessary() {
        let err = IssueNoteAwards::builder()
            .project(1)
            .note(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueNoteAwardsBuilderError, "issue",);
    }

    #[test]
    fn note_is_necessary() {
        let err = IssueNoteAwards::builder()
            .project(1)
            .issue(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, IssueNoteAwardsBuilderError, "note");
    }

    #[test]
    fn project_issue_and_note_are_sufficient() {
        IssueNoteAwards::builder()
            .project(1)
            .issue(1)
            .note(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/issues/1/notes/1/award_emoji")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = IssueNoteAwards::builder()
            .project("simple/project")
            .issue(1)
            .note(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
