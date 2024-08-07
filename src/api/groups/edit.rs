// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::time::Duration;

use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::groups::{
    BranchProtection, BranchProtectionDefaults, GroupProjectCreationAccessLevel,
    SharedRunnersMinutesLimit, SubgroupCreationAccessLevel,
};
use crate::api::projects::FeatureAccessLevel;
use crate::api::ParamValue;

/// Access levels for creating a project within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SharedRunnersSetting {
    /// All projects and subgroups can use shared runners.
    Enabled,
    /// Shared runners are not allowed, but subgroups can enable.
    DisabledWithOverride,
    /// Shared runners are not allowed for this group and all subgroups.
    DisableAndUnoverridable,
}

impl SharedRunnersSetting {
    fn as_str(self) -> &'static str {
        match self {
            SharedRunnersSetting::Enabled => "enabled",
            SharedRunnersSetting::DisabledWithOverride => "disabled_with_override",
            SharedRunnersSetting::DisableAndUnoverridable => "disabled_and_unoverridable",
        }
    }
}

impl ParamValue<'static> for SharedRunnersSetting {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Edit an existing group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditGroup<'a> {
    /// The group to edit.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// The name of the group.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The path of the group.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,
    /// A short description for the group.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Prevent adding members directly to projects within the group.
    #[builder(default)]
    membership_lock: Option<bool>,
    /// The visibility of the group.
    #[builder(default)]
    visibility: Option<VisibilityLevel>,
    /// Prevent sharing a project in this group with another group.
    #[builder(default)]
    share_with_group_lock: Option<bool>,
    /// Require two-factor authentication to be a member of this group.
    #[builder(default)]
    require_two_factor_authentication: Option<bool>,
    /// Time (in hours) for users to enable two-factor before enforcing it.
    #[builder(default)]
    two_factor_grace_period: Option<u64>,
    /// The access level to the group that is required to create new projects.
    #[builder(default)]
    project_creation_level: Option<GroupProjectCreationAccessLevel>,
    /// Default to Auto DevOps for new projects in the group.
    #[builder(default)]
    auto_devops_enabled: Option<bool>,
    /// The access level to the group that is required to create subgroups.
    #[builder(default)]
    subgroup_creation_level: Option<SubgroupCreationAccessLevel>,
    /// Disable email notifications from the group.
    #[builder(default)]
    #[deprecated(since = "0.1606.1", note = "use `emails_enabled` instead")]
    emails_disabled: Option<bool>,
    /// Enable email notifications from the group.
    #[builder(default)]
    emails_enabled: Option<bool>,
    // TODO: Figure out how to actually use this.
    // avatar   mixed   no  Image file for avatar of the group
    // avatar: ???,
    /// Disable group-wide mentions.
    #[builder(default)]
    mentions_disabled: Option<bool>,
    /// Disable sharing outside of the group hierarchy.
    ///
    /// Only available on top-level groups.
    #[builder(default)]
    prevent_sharing_groups_outside_hierarchy: Option<bool>,
    /// Whether `git-lfs` is enabled by default for projects within the group.
    #[builder(default)]
    lfs_enabled: Option<bool>,
    /// Whether access to the group may be requested.
    #[builder(default)]
    request_access_enabled: Option<bool>,
    /// The parent group ID (for subgroups).
    #[builder(default)]
    parent_id: Option<u64>,
    /// The default branch protection for projects within the group.
    #[builder(default)]
    default_branch_protection: Option<BranchProtection>,
    /// The default branch protection defaults for projects within the group.
    #[builder(default)]
    default_branch_protection_defaults: Option<BranchProtectionDefaults>,
    /// Shared runner settings for the group.
    #[builder(default)]
    shared_runners_setting: Option<SharedRunnersSetting>,
    /// Pipeline quota (in minutes) for the group on shared runners.
    #[builder(setter(into), default)]
    shared_runners_minutes_limit: Option<SharedRunnersMinutesLimit>,
    /// Pipeline quota excess (in minutes) for the group on shared runners.
    #[builder(default)]
    extra_shared_runners_minutes_limit: Option<u64>,
    /// The project id to load custom file templates from.
    #[builder(default)]
    file_template_project_id: Option<u64>,
    /// When enabled, users cannot fork projects from this group to other namespaces.
    #[builder(default)]
    prevent_forking_outside_group: Option<bool>,
    /// A set of IP addresses or IP ranges that are allowed to access the group.
    #[builder(setter(name = "_ip_restriction_ranges"), default, private)]
    ip_restriction_ranges: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// The wiki access level.
    #[builder(default)]
    wiki_access_level: Option<FeatureAccessLevel>,

    /// Maximum number of unique projects a user can download before being banned.
    ///
    /// Only supported on top-level groups.
    #[builder(default)]
    unique_project_download_limit: Option<u64>,
    /// The window (in seconds) where downloads will be counted.
    ///
    /// Only supported on top-level groups.
    #[builder(default)]
    unique_project_download_limit_interval: Option<Duration>,
    /// List of usernames excluded from the download limit.
    ///
    /// Only supported on top-level groups.
    #[builder(
        setter(name = "_unique_project_download_limit_allowlist"),
        default,
        private
    )]
    unique_project_download_limit_allowlist: BTreeSet<Cow<'a, str>>,
    /// List of user IDs that are emailed when a download limit is exceeded.
    ///
    /// Only supported on top-level groups.
    #[builder(
        setter(name = "_unique_project_download_limit_alertlist"),
        default,
        private
    )]
    unique_project_download_limit_alertlist: BTreeSet<u64>,
    /// Ban users from the group when they exceed the download limit.
    ///
    /// Only supported on top-level groups.
    #[builder(default)]
    auto_ban_user_on_excessive_projects_download: Option<bool>,
}

impl<'a> EditGroup<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditGroupBuilder<'a> {
        EditGroupBuilder::default()
    }
}

impl<'a> EditGroupBuilder<'a> {
    /// An IP address or IP range that is allowed to access the group.
    pub fn ip_restriction_range<R>(&mut self, range: R) -> &mut Self
    where
        R: Into<Cow<'a, str>>,
    {
        self.ip_restriction_ranges
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(range.into());
        self
    }

    /// A set of IP addresses or IP ranges that are allowed to access the group.
    pub fn ip_restriction_ranges<I, R>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = R>,
        R: Into<Cow<'a, str>>,
    {
        self.ip_restriction_ranges
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A username excluded from the download limit.
    pub fn unique_project_download_limit_allow<A>(&mut self, allow: A) -> &mut Self
    where
        A: Into<Cow<'a, str>>,
    {
        self.unique_project_download_limit_allowlist
            .get_or_insert_with(BTreeSet::new)
            .insert(allow.into());
        self
    }

    /// List of usernames excluded from the download limit.
    pub fn unique_project_download_limit_allow_users<I, A>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = A>,
        A: Into<Cow<'a, str>>,
    {
        self.unique_project_download_limit_allowlist
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A user ID that is emailed when a download limit is exceeded.
    pub fn unique_project_download_limit_alert(&mut self, alert: u64) -> &mut Self {
        self.unique_project_download_limit_alertlist
            .get_or_insert_with(BTreeSet::new)
            .insert(alert);
        self
    }

    /// List of user IDs that are emailed when a download limit is exceeded.
    pub fn unique_project_download_limit_alert_users<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.unique_project_download_limit_alertlist
            .get_or_insert_with(BTreeSet::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for EditGroup<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}", self.group).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("name", self.name.as_ref())
            .push_opt("path", self.path.as_ref())
            .push_opt("description", self.description.as_ref())
            .push_opt("membership_lock", self.membership_lock)
            .push_opt("visibility", self.visibility)
            .push_opt("share_with_group_lock", self.share_with_group_lock)
            .push_opt(
                "require_two_factor_authentication",
                self.require_two_factor_authentication,
            )
            .push_opt("two_factor_grace_period", self.two_factor_grace_period)
            .push_opt("project_creation_level", self.project_creation_level)
            .push_opt("auto_devops_enabled", self.auto_devops_enabled)
            .push_opt("subgroup_creation_level", self.subgroup_creation_level)
            .push_opt("emails_enabled", self.emails_enabled)
            .push_opt("mentions_disabled", self.mentions_disabled)
            .push_opt(
                "prevent_sharing_groups_outside_hierarchy",
                self.prevent_sharing_groups_outside_hierarchy,
            )
            .push_opt("lfs_enabled", self.lfs_enabled)
            .push_opt("request_access_enabled", self.request_access_enabled)
            .push_opt("parent_id", self.parent_id)
            .push_opt("default_branch_protection", self.default_branch_protection)
            .push_opt("shared_runners_setting", self.shared_runners_setting)
            .push_opt(
                "shared_runners_minutes_limit",
                self.shared_runners_minutes_limit,
            )
            .push_opt(
                "extra_shared_runners_minutes_limit",
                self.extra_shared_runners_minutes_limit,
            )
            .push_opt("file_template_project_id", self.file_template_project_id)
            .push_opt(
                "prevent_forking_outside_group",
                self.prevent_forking_outside_group,
            )
            .push_opt("ip_restriction_ranges", self.ip_restriction_ranges.as_ref())
            .push_opt("wiki_access_level", self.wiki_access_level)
            .push_opt(
                "unique_project_download_limit",
                self.unique_project_download_limit,
            )
            .push_opt(
                "unique_project_download_limit_interval_in_seconds",
                self.unique_project_download_limit_interval
                    .map(|interval| interval.as_secs()),
            )
            .extend(
                self.unique_project_download_limit_allowlist
                    .iter()
                    .map(|value| ("unique_project_download_limit_allowlist[]", value)),
            )
            .extend(
                self.unique_project_download_limit_alertlist
                    .iter()
                    .map(|&value| ("unique_project_download_limit_alertlist[]", value)),
            )
            .push_opt(
                "auto_ban_user_on_excessive_projects_download",
                self.auto_ban_user_on_excessive_projects_download,
            );

        if let Some(defaults) = self.default_branch_protection_defaults.as_ref() {
            defaults.add_query(&mut params);
        }

        #[allow(deprecated)]
        {
            params.push_opt("emails_disabled", self.emails_disabled);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use http::Method;

    use crate::api::common::VisibilityLevel;
    use crate::api::groups::{
        BranchProtection, BranchProtectionAccessLevel, BranchProtectionDefaults, EditGroup,
        EditGroupBuilderError, GroupProjectCreationAccessLevel, SharedRunnersMinutesLimit,
        SharedRunnersSetting, SubgroupCreationAccessLevel,
    };
    use crate::api::projects::FeatureAccessLevel;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn shared_runners_setting_as_str() {
        let items = &[
            (SharedRunnersSetting::Enabled, "enabled"),
            (
                SharedRunnersSetting::DisabledWithOverride,
                "disabled_with_override",
            ),
            (
                SharedRunnersSetting::DisableAndUnoverridable,
                "disabled_and_unoverridable",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn group_is_necessary() {
        let err = EditGroup::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditGroupBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        EditGroup::builder().group("group").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder().group("simple/group").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=name")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .name("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("path=path")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .path("path")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=description")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .description("description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_membership_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("membership_lock=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .membership_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("visibility=internal")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .visibility(VisibilityLevel::Internal)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_share_with_group_lock() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("share_with_group_lock=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .share_with_group_lock(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_require_two_factor_authentication() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("require_two_factor_authentication=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .require_two_factor_authentication(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_two_factor_grace_period() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("two_factor_grace_period=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .two_factor_grace_period(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_project_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("project_creation_level=maintainer")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .project_creation_level(GroupProjectCreationAccessLevel::Maintainer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_devops_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_devops_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .auto_devops_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_subgroup_creation_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("subgroup_creation_level=owner")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .subgroup_creation_level(SubgroupCreationAccessLevel::Owner)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn endpoint_emails_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("emails_disabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .emails_disabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_emails_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("emails_enabled=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .emails_enabled(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_mentions_disabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("mentions_disabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .mentions_disabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_prevent_sharing_groups_outside_hierarchy() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("prevent_sharing_groups_outside_hierarchy=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .prevent_sharing_groups_outside_hierarchy(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("lfs_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .lfs_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_request_access_enabled() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("request_access_enabled=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .request_access_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_parent_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("parent_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .parent_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection=2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection(BranchProtection::Full)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection_defaults_allowed_to_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection_defaults%5Ballowed_to_push%5D%5B%5D=30")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection_defaults(
                BranchProtectionDefaults::builder()
                    .allowed_to_push(BranchProtectionAccessLevel::Developer)
                    .allowed_to_push(BranchProtectionAccessLevel::Maintainer)
                    .not_allowed_to_push(BranchProtectionAccessLevel::Maintainer)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection_defaults_allow_force_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection_defaults%5Ballow_force_push%5D=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection_defaults(
                BranchProtectionDefaults::builder()
                    .allow_force_push(true)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection_defaults_allowed_to_merge() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection_defaults%5Ballowed_to_merge%5D%5B%5D=30")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection_defaults(
                BranchProtectionDefaults::builder()
                    .allowed_to_merge(BranchProtectionAccessLevel::Developer)
                    .allowed_to_merge(BranchProtectionAccessLevel::Maintainer)
                    .not_allowed_to_merge(BranchProtectionAccessLevel::Maintainer)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_default_branch_protection_defaults_developer_can_initial_push() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("default_branch_protection_defaults%5Bdeveloper_can_initial_push%5D=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .default_branch_protection_defaults(
                BranchProtectionDefaults::builder()
                    .developer_can_initial_push(true)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_setting() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_setting=disabled_with_override")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_setting(SharedRunnersSetting::DisabledWithOverride)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_minutes_limit=0")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_minutes_limit(SharedRunnersMinutesLimit::Unlimited)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit_into() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("shared_runners_minutes_limit=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_extra_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("extra_shared_runners_minutes_limit=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .extra_shared_runners_minutes_limit(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_file_template_project_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("file_template_project_id=1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .file_template_project_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_prevent_forking_outside_group() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("prevent_forking_outside_group=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .prevent_forking_outside_group(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ip_restriction_ranges() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ip_restriction_ranges=10.0.0.0%2F8%2C192.168.1.1%2C192.168.1.128%2F7")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .ip_restriction_range("10.0.0.0/8")
            .ip_restriction_ranges(["192.168.1.1", "192.168.1.128/7"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_wiki_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("wiki_access_level=disabled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .wiki_access_level(FeatureAccessLevel::Disabled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unique_project_download_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("unique_project_download_limit=100")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .unique_project_download_limit(100)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unique_project_download_limit_interval_in_seconds() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("unique_project_download_limit_interval_in_seconds=3600")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .unique_project_download_limit_interval(Duration::from_secs(3600))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unique_project_download_limit_allowlist() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "unique_project_download_limit_allowlist%5B%5D=auditor",
                "&unique_project_download_limit_allowlist%5B%5D=robot",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .unique_project_download_limit_allow("robot")
            .unique_project_download_limit_allow_users(["robot", "auditor"].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_unique_project_download_limit_alertlist() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "unique_project_download_limit_alertlist%5B%5D=1",
                "&unique_project_download_limit_alertlist%5B%5D=2",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .unique_project_download_limit_alert(2)
            .unique_project_download_limit_alert_users([2, 1].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auto_ban_user_on_excessive_projects_download() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup")
            .content_type("application/x-www-form-urlencoded")
            .body_str("auto_ban_user_on_excessive_projects_download=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditGroup::builder()
            .group("simple/group")
            .auto_ban_user_on_excessive_projects_download(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
