// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::variables::ProjectVariableType;

/// Add a variable to a pipeline schedule.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreatePipelineScheduleVariable<'a> {
    /// The project to add the variable to.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule to add the variable to.
    id: u64,
    /// The key of the variable
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// The value of a variable
    #[builder(setter(into))]
    value: Cow<'a, str>,
    /// The type of the variable.
    #[builder(default)]
    variable_type: Option<ProjectVariableType>,
}

impl<'a> CreatePipelineScheduleVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePipelineScheduleVariableBuilder<'a> {
        CreatePipelineScheduleVariableBuilder::default()
    }
}

impl<'a> Endpoint for CreatePipelineScheduleVariable<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/variables",
            self.project, self.id
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("key", &self.key)
            .push("value", &self.value)
            .push_opt("variable_type", self.variable_type);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::variables::create::{
        CreatePipelineScheduleVariable, CreatePipelineScheduleVariableBuilderError,
    };
    use crate::api::projects::variables::ProjectVariableType;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = CreatePipelineScheduleVariable::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn project_is_necessary() {
        let err = CreatePipelineScheduleVariable::builder()
            .id(10)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn id_is_necessary() {
        let err = CreatePipelineScheduleVariable::builder()
            .project(1)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleVariableBuilderError, "id");
    }

    #[test]
    fn key_is_necessary() {
        let err = CreatePipelineScheduleVariable::builder()
            .project(1)
            .id(10)
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePipelineScheduleVariableBuilderError, "key");
    }

    #[test]
    fn value_level_is_necessary() {
        let err = CreatePipelineScheduleVariable::builder()
            .project(1)
            .id(10)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePipelineScheduleVariableBuilderError,
            "value"
        );
    }

    #[test]
    fn sufficient_parameters() {
        CreatePipelineScheduleVariable::builder()
            .project(1)
            .id(1)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("key=testkey", "&value=testvalue"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipelineScheduleVariable::builder()
            .project("simple/project")
            .id(10)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_variable_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/variables")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "key=testkey",
                "&value=testvalue",
                "&variable_type=file"
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePipelineScheduleVariable::builder()
            .project("simple/project")
            .id(10)
            .key("testkey")
            .value("testvalue")
            .variable_type(ProjectVariableType::File)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
