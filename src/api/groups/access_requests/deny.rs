// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Deny a user access request to a group
#[derive(Debug, Builder, Clone)]
pub struct GroupAccessRequestsDeny<'a> {
    /// The group to query for membership.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// The user ID of the access requester
    user_id: u64,
}

impl<'a> GroupAccessRequestsDeny<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupAccessRequestsDenyBuilder<'a> {
        GroupAccessRequestsDenyBuilder::default()
    }
}

impl<'a> Endpoint for GroupAccessRequestsDeny<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/access_requests/{}", self.group, self.user_id).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::groups::access_requests::{
        GroupAccessRequestsDeny, GroupAccessRequestsDenyBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use http::Method;

    #[test]
    fn group_is_needed() {
        let err = GroupAccessRequestsDeny::builder()
            .user_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestsDenyBuilderError, "group");
    }

    #[test]
    fn user_id_is_needed() {
        let err = GroupAccessRequestsDeny::builder()
            .group(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestsDenyBuilderError, "user_id");
    }

    #[test]
    fn user_group_is_sufficient() {
        GroupAccessRequestsDeny::builder()
            .group(1)
            .user_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/simple%2Fgroup/access_requests/1")
            .method(Method::DELETE)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupAccessRequestsDeny::builder()
            .group("simple/group")
            .user_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
