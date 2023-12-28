// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Submit an access request to a group for the clients user token.
#[derive(Debug, Builder, Clone)]
pub struct GroupAccessRequest<'a> {
    /// The group to query for pipelines.
    #[builder(setter(into))]
    group: NameOrId<'a>,
}

impl<'a> GroupAccessRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupAccessRequestBuilder<'a> {
        GroupAccessRequestBuilder::default()
    }
}

impl<'a> Endpoint for GroupAccessRequest<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/access_requests", self.group).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::groups::access_requests::{GroupAccessRequest, GroupAccessRequestBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use http::Method;

    #[test]
    fn group_is_needed() {
        let err = GroupAccessRequest::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupAccessRequestBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupAccessRequest::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/simple%2Fgroup/access_requests")
            .method(Method::POST)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupAccessRequest::builder()
            .group("simple/group")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
