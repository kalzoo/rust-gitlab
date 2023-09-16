// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Manage push rules for a group.
///
/// See https://docs.gitlab.com/ee/api/groups.html#get-group-push-rules
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditGroupPushRule<'a> {
    /// The group to edit.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// Ensure commit messages match a given regular expression.
    #[builder(setter(into), default)]
    commit_message_regex: Option<Cow<'a, str>>,

    /// Ensure commit messages do not match a given regular expression.
    #[builder(setter(into), default)]
    commit_message_negative_regex: Option<Cow<'a, str>>,

    /// Restrict branch names to a given regular expression.
    #[builder(setter(into), default)]
    branch_name_regex: Option<Cow<'a, str>>,

    /// Do not allow users to delete a tag via `git push`.
    ///
    /// Users can still delete via the UI.
    #[builder(default)]
    deny_delete_tag: Option<bool>,

    /// Restrict commits by author (email) to existing GitLab users.
    #[builder(default)]
    member_check: Option<bool>,

    /// Reject commits with secrets.
    ///
    /// See [GitLab docs][gitlab-push-rules-secrets] for more details.
    ///
    /// [gitlab-push-rules-secrets]: https://docs.gitlab.com/ee/user/project/repository/push_rules.html#prevent-pushing-secrets-to-the-repository
    #[builder(default)]
    prevent_secrets: Option<bool>,

    /// Require commiter email addresses match a given regular expression.
    #[builder(setter(into), default)]
    author_email_regex: Option<Cow<'a, str>>,

    /// Reject files that match a given regular expression.
    #[builder(setter(into), default)]
    file_name_regex: Option<Cow<'a, str>>,

    /// Set the maximum size of a file (in megabytes).
    #[builder(default)]
    max_file_size: Option<u64>,

    /// Users can only push commits to this repository if the committer email is one of their own verified emails.
    #[builder(default)]
    commit_committer_check: Option<bool>,

    /// Reject commits that are not signed with a GPG key.
    #[builder(default)]
    reject_unsigned_commits: Option<bool>,
}

impl<'a> EditGroupPushRule<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditGroupPushRuleBuilder<'a> {
        EditGroupPushRuleBuilder::default()
    }
}

impl<'a> Endpoint for EditGroupPushRule<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/push_rule", self.group).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();
        params
            .push_opt("commit_message_regex", self.commit_message_regex.as_ref())
            .push_opt(
                "commit_message_negative_regex",
                self.commit_message_negative_regex.as_ref(),
            )
            .push_opt("branch_name_regex", self.branch_name_regex.as_ref())
            .push_opt("deny_delete_tag", self.deny_delete_tag)
            .push_opt("member_check", self.member_check)
            .push_opt("prevent_secrets", self.prevent_secrets)
            .push_opt("author_email_regex", self.author_email_regex.as_ref())
            .push_opt("file_name_regex", self.file_name_regex.as_ref())
            .push_opt("max_file_size", self.max_file_size)
            .push_opt("commit_committer_check", self.commit_committer_check)
            .push_opt("reject_unsigned_commits", self.reject_unsigned_commits);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::groups::push_rule::{EditGroupPushRule, EditGroupPushRuleBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_is_necessary() {
        let err = EditGroupPushRule::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditGroupPushRuleBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        EditGroupPushRule::builder().group("group").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/push_rule")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group("simple/group")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_author_email_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("author_email_regex=%40test.domain")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .author_email_regex("@test.domain")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_branch_name_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("branch_name_regex=main%28line%29%7Cmaster")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .branch_name_regex("main(line)|master")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_commit_committer_check() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("commit_committer_check=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .commit_committer_check(true)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_commit_message_negative_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("commit_message_negative_regex=%5Ba-z%5D%2B%5C%28%5Cw%2B%5C%29%3A.*")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .commit_message_negative_regex("[a-z]+\\(\\w+\\):.*")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_message_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("commit_message_regex=%5E%28%5BA-Z%5D%2B%5C-%5B0-9%5D%2B%29+%3A+%28.*%29")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .commit_message_regex("^([A-Z]+\\-[0-9]+) : (.*)")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_deny_delete_tag() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("deny_delete_tag=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .deny_delete_tag(true)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_file_name_regex() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("file_name_regex=%5E%28%5BA-Z%5D%2B%5C-%5B0-9%5D%2B%29+%3A+%28.*%29")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .file_name_regex("^([A-Z]+\\-[0-9]+) : (.*)")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_max_file_size() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("max_file_size=15")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .max_file_size(15)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_member_check() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("member_check=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .member_check(true)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_prevent_secrets() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("prevent_secrets=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .prevent_secrets(true)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_reject_unsigned_commits() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/x-www-form-urlencoded")
            .endpoint("groups/10/push_rule")
            .body_str("reject_unsigned_commits=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroupPushRule::builder()
            .group(10)
            .reject_unsigned_commits(true)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
