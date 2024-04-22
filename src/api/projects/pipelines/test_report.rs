// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for the test report of a pipeline.
#[derive(Debug, Builder, Clone)]
pub struct PipelineTestReport<'a> {
    /// The project of the pipelines.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline.
    pipeline: u64,
}

impl<'a> PipelineTestReport<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PipelineTestReportBuilder<'a> {
        PipelineTestReportBuilder::default()
    }
}

impl<'a> Endpoint for PipelineTestReport<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipelines/{}/test_report",
            self.project, self.pipeline,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::pipelines::test_report::{
        PipelineTestReport, PipelineTestReportBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_pipeline_are_needed() {
        let err = PipelineTestReport::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineTestReportBuilderError, "project");
    }

    #[test]
    fn project_is_needed() {
        let err = PipelineTestReport::builder()
            .pipeline(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PipelineTestReportBuilderError, "project");
    }

    #[test]
    fn pipeline_is_needed() {
        let err = PipelineTestReport::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, PipelineTestReportBuilderError, "pipeline");
    }

    #[test]
    fn project_and_pipeline_are_sufficient() {
        PipelineTestReport::builder()
            .project(1)
            .pipeline(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/pipelines/1/test_report")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineTestReport::builder()
            .project("simple/project")
            .pipeline(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
