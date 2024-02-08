// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::CommaSeparatedList;
use crate::api::endpoint_prelude::*;
use crate::api::runners::{RunnerStatus, RunnerType};

/// Query for all runners on an instance.
///
/// Note that this endpoint requires administrator privileges.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct AllRunners<'a> {
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

impl<'a> AllRunners<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AllRunnersBuilder<'a> {
        AllRunnersBuilder::default()
    }
}

impl<'a> AllRunnersBuilder<'a> {
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

impl<'a> Endpoint for AllRunners<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "runners/all".into()
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

impl<'a> Pageable for AllRunners<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::runners::{AllRunners, RunnerStatus, RunnerType};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn defaults_are_sufficient() {
        AllRunners::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_type() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .add_query_params(&[("type", "instance_type")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder()
            .type_(RunnerType::Instance)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .add_query_params(&[("status", "offline")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder()
            .status(RunnerStatus::Offline)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_paused() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .add_query_params(&[("paused", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder().paused(true).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .add_query_params(&[("tag_list", "tag2,tag1,tag3")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder()
            .tag("tag2")
            .tags(["tag1", "tag3"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_version_prefix() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/all")
            .add_query_params(&[("version_prefix", "16.8")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = AllRunners::builder()
            .version_prefix("16.8")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
