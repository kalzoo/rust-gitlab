// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Scopes for pipeline schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineScheduleScope {
    /// Active schedules.
    Active,
    /// Inactive schedules.
    Inactive,
}

impl PipelineScheduleScope {
    /// The scope as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            PipelineScheduleScope::Active => "active",
            PipelineScheduleScope::Inactive => "inactive",
        }
    }
}

impl ParamValue<'static> for PipelineScheduleScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for pipeline schedules on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PipelineSchedules<'a> {
    /// The project to query for pipeline schedules.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Filter schedules by its scope.
    #[builder(default)]
    scope: Option<PipelineScheduleScope>,
}

impl<'a> PipelineSchedules<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PipelineSchedulesBuilder<'a> {
        PipelineSchedulesBuilder::default()
    }
}

impl<'a> Endpoint for PipelineSchedules<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipeline_schedules", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("scope", self.scope);

        params
    }
}

impl<'a> Pageable for PipelineSchedules<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::pipeline_schedules::{
        PipelineScheduleScope, PipelineSchedules, PipelineSchedulesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn pipeline_schedule_scope_as_str() {
        let items = &[
            (PipelineScheduleScope::Active, "active"),
            (PipelineScheduleScope::Inactive, "inactive"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = PipelineSchedules::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PipelineSchedulesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        PipelineSchedules::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/pipeline_schedules")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineSchedules::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_scope() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/pipeline_schedules")
            .add_query_params(&[("scope", "inactive")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PipelineSchedules::builder()
            .project(1)
            .scope(PipelineScheduleScope::Inactive)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
