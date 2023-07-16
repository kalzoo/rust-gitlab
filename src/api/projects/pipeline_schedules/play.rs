// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Play of a pipeline schedule on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PlayPipelineSchedule<'a> {
    /// The project to take ownership of a pipeline schedule on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule id.
    id: u64,
}

impl<'a> PlayPipelineSchedule<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PlayPipelineScheduleBuilder<'a> {
        PlayPipelineScheduleBuilder::default()
    }
}

impl<'a> Endpoint for PlayPipelineSchedule<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/play",
            self.project, self.id
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::{
        PlayPipelineSchedule, PlayPipelineScheduleBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_id_are_needed() {
        let err = PlayPipelineSchedule::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PlayPipelineScheduleBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = PlayPipelineSchedule::builder().id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, PlayPipelineScheduleBuilderError, "project");
    }

    #[test]
    fn id_is_needed() {
        let err = PlayPipelineSchedule::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PlayPipelineScheduleBuilderError, "id");
    }

    #[test]
    fn project_and_id_are_sufficient() {
        PlayPipelineSchedule::builder()
            .project(1)
            .id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/play")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PlayPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
