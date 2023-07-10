// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a variable from a pipeline schedule.
#[derive(Debug, Builder, Clone)]
pub struct DeletePipelineScheduleVariable<'a> {
    /// The project to add the variable to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule to add the variable to.
    id: u64,
    /// The key of the variable.
    #[builder(setter(into))]
    key: Cow<'a, str>,
}

impl<'a> DeletePipelineScheduleVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeletePipelineScheduleVariableBuilder<'a> {
        DeletePipelineScheduleVariableBuilder::default()
    }
}

impl<'a> Endpoint for DeletePipelineScheduleVariable<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/variables/{}",
            self.project, self.id, self.key
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::variables::delete::{
        DeletePipelineScheduleVariable, DeletePipelineScheduleVariableBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = DeletePipelineScheduleVariable::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            DeletePipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn project_is_necessary() {
        let err = DeletePipelineScheduleVariable::builder()
            .id(10)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            DeletePipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn id_is_necessary() {
        let err = DeletePipelineScheduleVariable::builder()
            .project(1)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeletePipelineScheduleVariableBuilderError, "id");
    }

    #[test]
    fn key_is_necessary() {
        let err = DeletePipelineScheduleVariable::builder()
            .project(1)
            .id(10)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeletePipelineScheduleVariableBuilderError, "key");
    }

    #[test]
    fn sufficient_parameters() {
        DeletePipelineScheduleVariable::builder()
            .project(1)
            .id(1)
            .key("testkey")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/variables/testkey")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeletePipelineScheduleVariable::builder()
            .project("simple/project")
            .id(10)
            .key("testkey")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
