// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for pipelines created by a schedule.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PipelineSchedulePipelines<'a> {
    /// The project to query for pipeline schedules.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule id.
    id: u64,
}

impl<'a> PipelineSchedulePipelines<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PipelineSchedulePipelinesBuilder<'a> {
        PipelineSchedulePipelinesBuilder::default()
    }
}

impl<'a> Endpoint for PipelineSchedulePipelines<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/pipelines",
            self.project, self.id
        )
        .into()
    }
}

impl<'a> Pageable for PipelineSchedulePipelines<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::pipeline_schedules::{
        PipelineSchedulePipelines, PipelineSchedulePipelinesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_id_are_needed() {
        let err = PipelineSchedulePipelines::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineSchedulePipelinesBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = PipelineSchedulePipelines::builder()
            .id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PipelineSchedulePipelinesBuilderError, "project");
    }

    #[test]
    fn id_is_needed() {
        let err = PipelineSchedulePipelines::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PipelineSchedulePipelinesBuilderError, "id");
    }

    #[test]
    fn project_and_id_are_sufficient() {
        PipelineSchedulePipelines::builder()
            .project(1)
            .id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/pipelines")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineSchedulePipelines::builder()
            .project("simple/project")
            .id(10)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
