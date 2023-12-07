// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

use derive_builder::Builder;
use log::warn;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::repository::files::Encoding;
use crate::api::ParamValue;

/// All actions that can be performed in a commit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommitActionType {
    /// Create a file.
    // XXX(rust-1.62): use `#[default]`
    Create,
    /// delete a file.
    Delete,
    /// Move a file.
    Move,
    /// Change the contents of a file.
    Update,
    /// Change the execution permission on a file.
    Chmod,
}

impl Default for CommitActionType {
    fn default() -> Self {
        Self::Create
    }
}

impl CommitActionType {
    /// The string representation of the visibility level.
    pub fn as_str(self) -> &'static str {
        match self {
            CommitActionType::Create => "create",
            CommitActionType::Delete => "delete",
            CommitActionType::Move => "move",
            CommitActionType::Update => "update",
            CommitActionType::Chmod => "chmod",
        }
    }

    fn validate(self, builder: &CommitActionBuilder) -> Result<(), CommitActionValidationError> {
        if builder.content.is_some() {
            Ok(())
        } else {
            match self {
                Self::Create => Err(CommitActionValidationError::ContentRequiredByCreate),
                Self::Update => Err(CommitActionValidationError::ContentRequiredByUpdate),
                _ => Ok(()),
            }
        }
    }
}

impl ParamValue<'static> for CommitActionType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

const SAFE_ENCODING: Encoding = Encoding::Base64;

/// Action that is executed for a commit.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct CommitAction<'a> {
    /// The action to perform.
    action: CommitActionType,
    /// The path to the file.
    #[builder(setter(into))]
    file_path: Cow<'a, str>,
    /// Original full path to the file being moved.
    ///
    /// Only considered for `Move` action.
    #[builder(setter(into), default)]
    previous_path: Option<Cow<'a, str>>,
    /// File content, required for `Create` and `Update`.
    ///
    /// Move actions that do not specify content preserve the existing file content and any other
    /// value of content overwrites the file content.
    ///
    /// This will automatically be encoded according to the `encoding` parameter.
    #[builder(setter(into), default)]
    content: Option<Cow<'a, [u8]>>,
    /// The encoding to use for the content, text is default.
    ///
    /// Note that if `text` is requested and `content` contains non-UTF-8 content, a warning will
    /// be generated and a binary-safe encoding used instead.
    #[builder(default)]
    encoding: Option<Encoding>,
    /// Last known file commit ID.
    ///
    /// Only considered in `Update`, `Move`, and `Delete` actions.
    #[builder(setter(into), default)]
    last_commit_id: Option<Cow<'a, str>>,
    /// When true/false enables/disables the execute flag on the file.
    ///
    /// Only considered for the `Chmod` action.
    #[builder(default)]
    execute_filemode: Option<bool>,
}

impl<'a> CommitAction<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitActionBuilder<'a> {
        CommitActionBuilder::default()
    }

    fn add_query<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push("actions[][action]", self.action.as_value())
            .push("actions[][file_path]", self.file_path.as_value())
            .push_opt("actions[][previous_path]", self.previous_path.as_ref())
            .push_opt(
                "actions[][content]",
                self.content.as_ref().map(|content| {
                    let str_content = str::from_utf8(content);
                    let needs_encoding = str_content.is_err();
                    let encoding = self.encoding.unwrap_or_default();
                    let actual_encoding = if needs_encoding && !encoding.is_binary_safe() {
                        warn!(
                            "forcing the encoding to {} due to utf-8 unsafe content",
                            SAFE_ENCODING.as_str(),
                        );
                        SAFE_ENCODING
                    } else {
                        encoding
                    };
                    actual_encoding.encode(str_content.ok(), content)
                }),
            )
            .push_opt("actions[][encoding]", self.encoding)
            .push_opt("actions[][last_commit_id]", self.last_commit_id.as_ref())
            .push_opt("actions[][execute_filemode]", self.execute_filemode);
    }
}

static CONTENT_REQUIRED_CREATE: &str = "content is required for create.";
static CONTENT_REQUIRED_UPDATE: &str = "content is required for update.";

#[non_exhaustive]
enum CommitActionValidationError {
    ContentRequiredByCreate,
    ContentRequiredByUpdate,
}

impl From<CommitActionValidationError> for CommitActionBuilderError {
    fn from(validation_error: CommitActionValidationError) -> Self {
        match validation_error {
            CommitActionValidationError::ContentRequiredByCreate => {
                CommitActionBuilderError::ValidationError(CONTENT_REQUIRED_CREATE.into())
            },
            CommitActionValidationError::ContentRequiredByUpdate => {
                CommitActionBuilderError::ValidationError(CONTENT_REQUIRED_UPDATE.into())
            },
        }
    }
}

impl<'a> CommitActionBuilder<'a> {
    fn validate(&self) -> Result<(), CommitActionValidationError> {
        if let Some(ref action) = &self.action {
            action.validate(self)?;
        }

        Ok(())
    }
}

/// Create a new commit for a project on a branch.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct CreateCommit<'a> {
    /// The ID or URL-encoded path of the project
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// Name of the branch to commit into.
    ///
    /// To create a new branch, also provide either `start_branch` or `start_sha`, and (optionally)
    /// `start_project`.
    #[builder(setter(into))]
    branch: Cow<'a, str>,
    /// Commit message.
    #[builder(setter(into))]
    commit_message: Cow<'a, str>,
    /// Name of the branch to start the new branch from.
    #[builder(setter(into), default)]
    start_branch: Option<Cow<'a, str>>,
    /// SHA of the commit to start the new branch from.
    #[builder(setter(into), default)]
    start_sha: Option<Cow<'a, str>>,
    /// The project path or ID of the project to start the new branch from.
    ///
    /// Defaults to the value of `project`.
    #[builder(setter(into), default)]
    start_project: Option<NameOrId<'a>>,
    /// An array of action hashes to commit as a batch.
    #[builder(setter(name = "_actions"), private)]
    actions: Vec<CommitAction<'a>>,
    /// Specify the commit author's email address.
    #[builder(setter(into), default)]
    author_email: Option<Cow<'a, str>>,
    /// Specify the commit author's name.
    #[builder(setter(into), default)]
    author_name: Option<Cow<'a, str>>,
    /// Include commit stats.
    ///
    /// Default to `true`.
    #[builder(default)]
    stats: Option<bool>,
    /// When `true`, overwrites the target branch with a new commit based on the `start_branch` or
    /// `start_sha`.
    #[builder(default)]
    force: Option<bool>,
}

impl<'a> CreateCommit<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateCommitBuilder<'a> {
        CreateCommitBuilder::default()
    }
}

#[non_exhaustive]
enum CreateCommitValidationError {
    AtMostOneStartItem,
}

static AT_MOST_ONE_START_ITEM: &str = "Specify either start_sha or start_branch, not both";

impl From<CreateCommitValidationError> for CreateCommitBuilderError {
    fn from(validation_error: CreateCommitValidationError) -> Self {
        match validation_error {
            CreateCommitValidationError::AtMostOneStartItem => {
                CreateCommitBuilderError::ValidationError(AT_MOST_ONE_START_ITEM.into())
            },
        }
    }
}

impl<'a> CreateCommitBuilder<'a> {
    /// Add an action.
    pub fn action(&mut self, action: CommitAction<'a>) -> &mut Self {
        self.actions.get_or_insert(Vec::new()).push(action);
        self
    }

    /// Add multiple actions.
    pub fn actions<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = CommitAction<'a>>,
    {
        self.actions.get_or_insert(Vec::new()).extend(iter);
        self
    }

    fn validate(&self) -> Result<(), CreateCommitValidationError> {
        let have_start_branch = self
            .start_branch
            .as_ref()
            .map(Option::is_some)
            .unwrap_or(false);
        let have_start_sha = self
            .start_sha
            .as_ref()
            .map(Option::is_some)
            .unwrap_or(false);
        if have_start_branch && have_start_sha {
            return Err(CreateCommitValidationError::AtMostOneStartItem);
        }

        Ok(())
    }
}

impl<'a> Endpoint for CreateCommit<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/commits", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("branch", self.branch.as_ref())
            .push("commit_message", self.commit_message.as_ref())
            .push_opt("start_branch", self.start_branch.as_ref())
            .push_opt("start_sha", self.start_sha.as_ref())
            .push_opt("start_project", self.start_project.as_ref())
            .push_opt("author_email", self.author_email.as_ref())
            .push_opt("author_name", self.author_name.as_ref())
            .push_opt("stats", self.stats)
            .push_opt("force", self.force);

        for action in self.actions.iter() {
            action.add_query(&mut params);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{self, Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::*;

    #[test]
    fn action_action_type_required() {
        let err = CommitAction::builder()
            .file_path("path/to/file")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CommitActionBuilderError, "action");
    }

    #[test]
    fn action_file_path_required() {
        let err = CommitAction::builder()
            .action(CommitActionType::Create)
            .content(&b"content"[..])
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CommitActionBuilderError, "file_path");
    }

    #[test]
    fn action_content_required_for_create() {
        let action = CommitAction::builder()
            .action(CommitActionType::Create)
            .file_path("path/to/file")
            .build();

        if let Err(msg) = action {
            assert_eq!(msg.to_string(), CONTENT_REQUIRED_CREATE)
        } else {
            panic!("unexpected error (expected to be missing content)")
        }
    }

    #[test]
    fn action_content_required_for_update() {
        let action = CommitAction::builder()
            .action(CommitActionType::Update)
            .file_path("path/to/file")
            .build();

        if let Err(msg) = action {
            assert_eq!(msg.to_string(), CONTENT_REQUIRED_UPDATE)
        } else {
            panic!("unexpected error (expected to be missing content)")
        }
    }

    #[test]
    fn project_is_required() {
        let err = CreateCommit::builder()
            .branch("source")
            .commit_message("msg")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitBuilderError, "project");
    }

    #[test]
    fn branch_is_required() {
        let err = CreateCommit::builder()
            .project(1)
            .commit_message("msg")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitBuilderError, "branch");
    }

    #[test]
    fn commit_message_is_required() {
        let err = CreateCommit::builder()
            .project(1)
            .branch("source")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitBuilderError, "commit_message");
    }

    #[test]
    fn actions_required() {
        let err = CreateCommit::builder()
            .project(1)
            .branch("source")
            .commit_message("msg")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateCommitBuilderError, "actions");
    }

    #[test]
    fn project_branch_msg_and_action_sufficent() {
        CreateCommit::builder()
            .project(1)
            .branch("source")
            .commit_message("msg")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
                "&actions%5B%5D%5Baction%5D=delete",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar2",
                "&actions%5B%5D%5Baction%5D=move",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar3",
                "&actions%5B%5D%5Bprevious_path%5D=foo%2Fbar4",
                "&actions%5B%5D%5Bcontent%5D=content",
                "&actions%5B%5D%5Baction%5D=update",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar5",
                "&actions%5B%5D%5Bcontent%5D=content",
                "&actions%5B%5D%5Baction%5D=chmod",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar5",
                "&actions%5B%5D%5Bexecute_filemode%5D=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .commit_message("message")
            .actions(vec![
                CommitAction::builder()
                    .action(CommitActionType::Create)
                    .file_path("foo/bar")
                    .content(&b"content"[..])
                    .build()
                    .unwrap(),
                CommitAction::builder()
                    .action(CommitActionType::Delete)
                    .file_path("foo/bar2")
                    .build()
                    .unwrap(),
                CommitAction::builder()
                    .action(CommitActionType::Move)
                    .file_path("foo/bar3")
                    .previous_path("foo/bar4")
                    .content(&b"content"[..])
                    .build()
                    .unwrap(),
                CommitAction::builder()
                    .action(CommitActionType::Update)
                    .file_path("foo/bar5")
                    .content(&b"content"[..])
                    .build()
                    .unwrap(),
                CommitAction::builder()
                    .action(CommitActionType::Chmod)
                    .file_path("foo/bar5")
                    .execute_filemode(true)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_branch() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&start_branch=start",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .commit_message("message")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .start_branch("start")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_sha() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=new-branch",
                "&commit_message=message",
                "&start_sha=40b35d15a129e75500bbf3d5db779b6f29376d1a",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("new-branch")
            .start_sha("40b35d15a129e75500bbf3d5db779b6f29376d1a")
            .commit_message("message")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_start_branch_and_start_sha() {
        let err = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .commit_message("message")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .start_branch("start")
            .start_sha("start")
            .build()
            .unwrap_err();

        assert_eq!(err.to_string(), AT_MOST_ONE_START_ITEM);
    }

    #[test]
    fn endpoint_start_project() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=new-branch",
                "&commit_message=message",
                "&start_project=400",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("new-branch")
            .start_project(400)
            .commit_message("message")
            .actions([CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_email() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&author_email=me%40mail.com",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .author_email("me@mail.com")
            .commit_message("message")
            .actions(vec![CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&author_name=me",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .author_name("me")
            .commit_message("message")
            .actions(vec![CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_stats() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&stats=true",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .stats(true)
            .commit_message("message")
            .actions(vec![CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_force() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/repository/commits")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "branch=master",
                "&commit_message=message",
                "&force=true",
                "&actions%5B%5D%5Baction%5D=create",
                "&actions%5B%5D%5Bfile_path%5D=foo%2Fbar",
                "&actions%5B%5D%5Bcontent%5D=content",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateCommit::builder()
            .project("simple/project")
            .branch("master")
            .force(true)
            .commit_message("message")
            .actions(vec![CommitAction::builder()
                .action(CommitActionType::Create)
                .file_path("foo/bar")
                .content(&b"content"[..])
                .build()
                .unwrap()])
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
