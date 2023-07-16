// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::variables::ProjectVariableType;

/// Edit a variable in a pipeline schedule.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditPipelineScheduleVariable<'a> {
    /// The project to edit the variable within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule to edit the variable within.
    id: u64,
    /// The key of the variable.
    #[builder(setter(into))]
    key: Cow<'a, str>,
    /// The value of a variable.
    #[builder(setter(into))]
    value: Cow<'a, str>,
    /// The type of the variable.
    #[builder(default)]
    variable_type: Option<ProjectVariableType>,
}

impl<'a> EditPipelineScheduleVariable<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditPipelineScheduleVariableBuilder<'a> {
        EditPipelineScheduleVariableBuilder::default()
    }
}

impl<'a> Endpoint for EditPipelineScheduleVariable<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/variables/{}",
            self.project, self.id, self.key
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("value", &self.value)
            .push_opt("variable_type", self.variable_type);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::variables::edit::{
        EditPipelineScheduleVariable, EditPipelineScheduleVariableBuilderError,
    };
    use crate::api::projects::variables::ProjectVariableType;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = EditPipelineScheduleVariable::builder().build().unwrap_err();
        crate::test::assert_missing_field!(
            err,
            EditPipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn project_is_necessary() {
        let err = EditPipelineScheduleVariable::builder()
            .id(10)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            EditPipelineScheduleVariableBuilderError,
            "project"
        );
    }

    #[test]
    fn id_is_necessary() {
        let err = EditPipelineScheduleVariable::builder()
            .project(1)
            .key("testkey")
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleVariableBuilderError, "id");
    }

    #[test]
    fn key_is_necessary() {
        let err = EditPipelineScheduleVariable::builder()
            .project(1)
            .id(10)
            .value("testvalue")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleVariableBuilderError, "key");
    }

    #[test]
    fn value_level_is_necessary() {
        let err = EditPipelineScheduleVariable::builder()
            .project(1)
            .id(10)
            .key("testkey")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleVariableBuilderError, "value");
    }

    #[test]
    fn sufficient_parameters() {
        EditPipelineScheduleVariable::builder()
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
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str("value=testvalue")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineScheduleVariable::builder()
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
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/variables/testkey")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("value=testvalue", "&variable_type=file"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineScheduleVariable::builder()
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
