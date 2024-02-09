// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::CommaSeparatedList;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Access levels of runners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RunnerAccessLevel {
    /// Run jobs from any pipeline.
    NotProtected,
    /// Only run jobs from protected refs.
    RefProtected,
}

impl RunnerAccessLevel {
    /// The runner type as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            Self::NotProtected => "not_protected",
            Self::RefProtected => "ref_protected",
        }
    }
}

impl ParamValue<'static> for RunnerAccessLevel {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Edit the details of a runner.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditRunner<'a> {
    /// The ID of the runner.
    runner: u64,

    /// The description of the runner.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Whether the runner should ignore new jobs or not.
    #[builder(default)]
    paused: Option<bool>,
    /// Set the tags for the runner.
    #[builder(setter(name = "_tag_list"), default, private)]
    tag_list: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// Whether the runner can execute untagged jobs or not.
    #[builder(default)]
    run_untagged: Option<bool>,
    /// Whether the runner is locked or not.
    #[builder(default)]
    locked: Option<bool>,
    /// The access level of the runner.
    #[builder(default)]
    access_level: Option<RunnerAccessLevel>,
    /// The maximum timeout allowed on the runner (in seconds).
    #[builder(default)]
    maximum_timeout: Option<u64>,
}

impl<'a> EditRunner<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditRunnerBuilder<'a> {
        EditRunnerBuilder::default()
    }
}

impl<'a> EditRunnerBuilder<'a> {
    /// Add a tag to the runner.
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

    /// Add multiple tags to the runner.
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

impl<'a> Endpoint for EditRunner<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("runners/{}", self.runner).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("description", self.description.as_ref())
            .push_opt("paused", self.paused)
            .push_opt("tag_list", self.tag_list.as_ref())
            .push_opt("run_untagged", self.run_untagged)
            .push_opt("locked", self.locked)
            .push_opt("access_level", self.access_level)
            .push_opt("maximum_timeout", self.maximum_timeout);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{EditRunner, EditRunnerBuilderError, RunnerAccessLevel};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn runner_access_level_as_str() {
        let items = &[
            (RunnerAccessLevel::NotProtected, "not_protected"),
            (RunnerAccessLevel::RefProtected, "ref_protected"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn runner_is_required() {
        let err = EditRunner::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditRunnerBuilderError, "runner");
    }

    #[test]
    fn runner_is_sufficient() {
        EditRunner::builder().runner(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder().runner(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=desc")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .description("desc")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_paused() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("paused=true")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .paused(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("tag_list=tag2%2Ctag1%2Ctag3")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .tag("tag2")
            .tags(["tag1", "tag3"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_run_untagged() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("run_untagged=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .run_untagged(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_locked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("locked=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .locked(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("access_level=ref_protected")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .access_level(RunnerAccessLevel::RefProtected)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_maximum_timeout() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("runners/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("maximum_timeout=3600")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditRunner::builder()
            .runner(1)
            .maximum_timeout(3600)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
