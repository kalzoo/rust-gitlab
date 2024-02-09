// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::runners::{RunnerStatus, RunnerType};

/// Query for runners on a group.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct GroupRunners<'a> {
    /// The ID or URL-encoded path of the group.
    #[builder(setter(into))]
    group: NameOrId<'a>,

    /// Filter runners by type.
    #[builder(default)]
    type_: Option<RunnerType>,
    /// Filter runners by status.
    #[builder(default)]
    status: Option<RunnerStatus>,
    /// Filter runners by paused status.
    #[builder(default)]
    paused: Option<bool>,
    /// Filter results by tags.
    #[builder(setter(name = "_tag_list"), default, private)]
    tag_list: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// Filter runners by version prefix.
    #[builder(setter(into), default)]
    version_prefix: Option<Cow<'a, str>>,
}

impl<'a> GroupRunners<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupRunnersBuilder<'a> {
        GroupRunnersBuilder::default()
    }
}

impl<'a> GroupRunnersBuilder<'a> {
    /// Add a tag to filter by.
    pub fn tag<T>(&mut self, tag: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(tag.into());
        self
    }

    /// Add multiple tags to filter by.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(|t| t.into()));
        self
    }
}

impl<'a> Endpoint for GroupRunners<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/runners", self.group).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("type", self.type_)
            .push_opt("status", self.status)
            .push_opt("paused", self.paused)
            .push_opt("tag_list", self.tag_list.as_ref())
            .push_opt("version_prefix", self.version_prefix.as_ref());

        params
    }
}

impl<'a> Pageable for GroupRunners<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::groups::runners::{GroupRunners, GroupRunnersBuilderError};
    use crate::api::runners::{RunnerStatus, RunnerType};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn group_is_necessary() {
        let err = GroupRunners::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, GroupRunnersBuilderError, "group");
    }

    #[test]
    fn group_is_sufficient() {
        GroupRunners::builder().group(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder().group(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_type() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .add_query_params(&[("type", "instance_type")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder()
            .group(1)
            .type_(RunnerType::Instance)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .add_query_params(&[("status", "offline")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder()
            .group(1)
            .status(RunnerStatus::Offline)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_paused() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .add_query_params(&[("paused", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder()
            .group(1)
            .paused(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .add_query_params(&[("tag_list", "tag2,tag1,tag3")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder()
            .group(1)
            .tag("tag2")
            .tags(["tag1", "tag3"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_version_prefix() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1/runners")
            .add_query_params(&[("version_prefix", "16.8")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GroupRunners::builder()
            .group(1)
            .version_prefix("16.8")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
