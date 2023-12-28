// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Access levels for groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum GroupAccessLevel {
    /// Anonymous access.
    Anonymous,
    /// Minimal access.
    Minimal,
    /// Guest access (can see the project).
    Guest,
    /// Reporter access (can open issues).
    Reporter,
    /// Developer access (can push branches, handle issues and merge requests).
    Developer,
    /// Maintainer access (can push to protected branches).
    Maintainer,
    /// Owner access (full rights).
    Owner,
}

impl GroupAccessLevel {
    /// The string representation of the access level.
    pub fn as_str(self) -> &'static str {
        match self {
            GroupAccessLevel::Owner => "owner",
            GroupAccessLevel::Maintainer => "maintainer",
            GroupAccessLevel::Developer => "developer",
            GroupAccessLevel::Reporter => "reporter",
            GroupAccessLevel::Guest => "guest",
            GroupAccessLevel::Minimal => "minimal",
            GroupAccessLevel::Anonymous => "anonymous",
        }
    }

    /// The integer representation of the access level.
    pub fn as_u64(self) -> u64 {
        match self {
            GroupAccessLevel::Owner => 50,
            GroupAccessLevel::Maintainer => 40,
            GroupAccessLevel::Developer => 30,
            GroupAccessLevel::Reporter => 20,
            GroupAccessLevel::Guest => 10,
            GroupAccessLevel::Minimal => 5,
            GroupAccessLevel::Anonymous => 0,
        }
    }
}

/// Submit approval for a user access request to a group
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct GroupAccessRequestsApprove<'a> {
    /// The group to query for membership.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// The user ID of the access requester
    user_id: u64,

    /// A valid access level (defaults: the Developer role)
    #[builder(setter(into), default)]
    access_level: Option<GroupAccessLevel>,
}

impl<'a> GroupAccessRequestsApprove<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupAccessRequestsApproveBuilder<'a> {
        GroupAccessRequestsApproveBuilder::default()
    }
}

impl<'a> Endpoint for GroupAccessRequestsApprove<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "groups/{}/access_requests/{}/approve",
            self.group, self.user_id,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt(
            "access_level",
            self.access_level.map(|level| level.as_u64()),
        );

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::common::AccessLevel;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use crate::api::groups::access_requests::{
        GroupAccessLevel, GroupAccessRequestsApprove, GroupAccessRequestsApproveBuilderError,
    };

    use http::Method;

    #[test]
    fn common_access_level_consisent() {
        let items = &[
            (GroupAccessLevel::Anonymous, AccessLevel::Anonymous),
            (GroupAccessLevel::Minimal, AccessLevel::Minimal),
            (GroupAccessLevel::Guest, AccessLevel::Guest),
            (GroupAccessLevel::Reporter, AccessLevel::Reporter),
            (GroupAccessLevel::Developer, AccessLevel::Developer),
            (GroupAccessLevel::Maintainer, AccessLevel::Maintainer),
            (GroupAccessLevel::Owner, AccessLevel::Owner),
        ];

        for (g, c) in items {
            assert_eq!(g.as_str(), c.as_str());
            assert_eq!(g.as_u64(), c.as_u64());
        }
    }

    #[test]
    fn access_level_as_str() {
        let items = &[
            (GroupAccessLevel::Anonymous, "anonymous", 0),
            (GroupAccessLevel::Minimal, "minimal", 5),
            (GroupAccessLevel::Guest, "guest", 10),
            (GroupAccessLevel::Reporter, "reporter", 20),
            (GroupAccessLevel::Developer, "developer", 30),
            (GroupAccessLevel::Maintainer, "maintainer", 40),
            (GroupAccessLevel::Owner, "owner", 50),
        ];

        for (i, s, u) in items {
            assert_eq!(i.as_str(), *s);
            assert_eq!(i.as_u64(), *u);
        }
    }

    #[test]
    fn access_level_ordering() {
        let items = &[
            GroupAccessLevel::Anonymous,
            GroupAccessLevel::Minimal,
            GroupAccessLevel::Guest,
            GroupAccessLevel::Reporter,
            GroupAccessLevel::Developer,
            GroupAccessLevel::Maintainer,
            GroupAccessLevel::Owner,
        ];

        let mut last = None;
        for item in items {
            if let Some(prev) = last {
                assert!(prev < item);
            }
            last = Some(item);
        }
    }

    #[test]
    fn group_is_needed() {
        let err = GroupAccessRequestsApprove::builder()
            .user_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestsApproveBuilderError, "group");
    }

    #[test]
    fn user_id_is_needed() {
        let err = GroupAccessRequestsApprove::builder()
            .group(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestsApproveBuilderError, "user_id");
    }

    #[test]
    fn user_group_is_sufficient() {
        GroupAccessRequestsApprove::builder()
            .group(1)
            .user_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/access_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupAccessRequestsApprove::builder()
            .group("simple/group")
            .user_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("groups/simple%2Fgroup/access_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .body_str("access_level=30")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupAccessRequestsApprove::builder()
            .group("simple/group")
            .user_id(1)
            .access_level(GroupAccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
