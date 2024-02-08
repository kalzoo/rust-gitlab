// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{CommaSeparatedList, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::runners::{RunnerStatus, RunnerType};

/// Query for runners on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectRunners<'a> {
    /// The ID or URL-encoded path of the project.
    #[builder(setter(into))]
    project: NameOrId<'a>,

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

impl<'a> ProjectRunners<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectRunnersBuilder<'a> {
        ProjectRunnersBuilder::default()
    }
}

impl<'a> ProjectRunnersBuilder<'a> {
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

impl<'a> Endpoint for ProjectRunners<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/runners", self.project).into()
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

impl<'a> Pageable for ProjectRunners<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::runners::{ProjectRunners, ProjectRunnersBuilderError};
    use crate::api::runners::{RunnerStatus, RunnerType};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = ProjectRunners::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectRunnersBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectRunners::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder().project(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_type() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .add_query_params(&[("type", "instance_type")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder()
            .project(1)
            .type_(RunnerType::Instance)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .add_query_params(&[("status", "offline")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder()
            .project(1)
            .status(RunnerStatus::Offline)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_paused() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .add_query_params(&[("paused", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder()
            .project(1)
            .paused(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .add_query_params(&[("tag_list", "tag2,tag1,tag3")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder()
            .project(1)
            .tag("tag2")
            .tags(["tag1", "tag3"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_version_prefix() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/runners")
            .add_query_params(&[("version_prefix", "16.8")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectRunners::builder()
            .project(1)
            .version_prefix("16.8")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
