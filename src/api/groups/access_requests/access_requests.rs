// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for access requests to a group.
#[derive(Debug, Builder, Clone)]
pub struct GroupAccessRequests<'a> {
    /// The group to query for pipelines.
    #[builder(setter(into))]
    group: NameOrId<'a>,
}

impl<'a> GroupAccessRequests<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupAccessRequestsBuilder<'a> {
        GroupAccessRequestsBuilder::default()
    }
}

impl<'a> Endpoint for GroupAccessRequests<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/access_requests", self.group).into()
    }
}

impl<'a> Pageable for GroupAccessRequests<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::groups::access_requests::{
        GroupAccessRequests, GroupAccessRequestsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_is_needed() {
        let err = GroupAccessRequests::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestsBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupAccessRequests::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/simple%2Fgroup/access_requests")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupAccessRequests::builder()
            .group("simple/group")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
