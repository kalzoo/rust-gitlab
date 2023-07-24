// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query a members of a group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct GroupMembers<'a> {
    /// The group to query for membership.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// A search string to filter members by.
    #[builder(setter(into), default)]
    query: Option<Cow<'a, str>>,
    /// A search string to filter members by.
    #[builder(setter(name = "_user_ids"), default, private)]
    user_ids: BTreeSet<u64>,
    /// Skip certain user results.
    #[builder(setter(name = "_skip_users"), default, private)]
    skip_users: BTreeSet<u64>,
    /// Show seat information for users.
    #[builder(default)]
    show_seat_info: Option<bool>,
}

impl<'a> GroupMembers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupMembersBuilder<'a> {
        GroupMembersBuilder::default()
    }
}

impl<'a> GroupMembersBuilder<'a> {
    /// Filter results by the given user ID.
    pub fn user_id(&mut self, user_id: u64) -> &mut Self {
        self.user_ids
            .get_or_insert_with(BTreeSet::new)
            .insert(user_id);
        self
    }

    /// Filter results by the given user IDs.
    pub fn user_ids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.user_ids.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Skip a certain user.
    pub fn skip_user(&mut self, user_id: u64) -> &mut Self {
        self.skip_users
            .get_or_insert_with(BTreeSet::new)
            .insert(user_id);
        self
    }

    /// Skip a set of users.
    pub fn skip_users<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.skip_users
            .get_or_insert_with(BTreeSet::new)
            .extend(iter);
        self
    }
}

impl<'a> Endpoint for GroupMembers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/members", self.group).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("query", self.query.as_ref())
            .extend(self.user_ids.iter().map(|&value| ("user_ids[]", value)))
            .extend(self.skip_users.iter().map(|&value| ("skip_users[]", value)))
            .push_opt("show_seat_info", self.show_seat_info);

        params
    }
}

impl<'a> Pageable for GroupMembers<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::groups::members::{GroupMembers, GroupMembersBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_is_needed() {
        let err = GroupMembers::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupMembersBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupMembers::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMembers::builder()
            .group("group/subgroup")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_query() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members")
            .add_query_params(&[("query", "search")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMembers::builder()
            .group("group/subgroup")
            .query("search")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_user_ids() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members")
            .add_query_params(&[("user_ids[]", "1"), ("user_ids[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMembers::builder()
            .group("group/subgroup")
            .user_id(1)
            .user_ids([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_users() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members")
            .add_query_params(&[("skip_users[]", "1"), ("skip_users[]", "2")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMembers::builder()
            .group("group/subgroup")
            .skip_user(1)
            .skip_users([1, 2].iter().copied())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_show_seat_info() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/group%2Fsubgroup/members")
            .add_query_params(&[("show_seat_info", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupMembers::builder()
            .group("group/subgroup")
            .show_seat_info(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
