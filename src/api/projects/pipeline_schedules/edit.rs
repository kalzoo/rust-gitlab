// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::pipeline_schedules::{PipelineScheduleCron, PipelineScheduleTimeZone};

/// Edit a pipeline schedule on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EditPipelineSchedule<'a> {
    /// The project to edit the pipeline schedule within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the pipeline schedule.
    id: u64,

    /// The description of the pipeline schedule.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// The ref to create the pipeline schedule for.
    #[builder(setter(into), default)]
    ref_: Option<Cow<'a, str>>,
    /// The `cron` schedule.
    #[builder(default)]
    cron: Option<PipelineScheduleCron>,
    /// The timezone to use.
    ///
    /// Defaults to `UTC`.
    #[builder(default)]
    cron_timezone: Option<PipelineScheduleTimeZone<'a>>,
    /// Whether the pipeline is active or not.
    #[builder(default)]
    active: Option<bool>,
}

impl<'a> EditPipelineSchedule<'a> {
    /// Edit a builder for the endpoint.
    pub fn builder() -> EditPipelineScheduleBuilder<'a> {
        EditPipelineScheduleBuilder::default()
    }
}

impl<'a> Endpoint for EditPipelineSchedule<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/pipeline_schedules/{}", self.project, self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("description", self.description.as_ref())
            .push_opt("ref", self.ref_.as_ref())
            .push_opt("cron", self.cron.as_ref())
            .push_opt("cron_timezone", self.cron_timezone.as_ref())
            .push_opt("active", self.active);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::{
        EditPipelineSchedule, EditPipelineScheduleBuilderError, PipelineScheduleCron,
        PipelineScheduleTimeZone,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_id_are_necessary() {
        let err = EditPipelineSchedule::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditPipelineSchedule::builder().id(10).build().unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleBuilderError, "project");
    }

    #[test]
    fn id_is_necessary() {
        let err = EditPipelineSchedule::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditPipelineScheduleBuilderError, "id");
    }

    #[test]
    fn project_and_id_are_sufficient() {
        EditPipelineSchedule::builder()
            .project(1)
            .id(10)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .body_str("description=desc")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .description("desc")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .body_str("ref=master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .ref_("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_cron() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .body_str("cron=0+1+*+*+*")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .cron(PipelineScheduleCron::new("0 1 * * *").unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_cron_timezone() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .body_str("cron_timezone=Newfoundland")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .cron_timezone(PipelineScheduleTimeZone::Newfoundland)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_active() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10")
            .content_type("application/x-www-form-urlencoded")
            .body_str("active=false")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditPipelineSchedule::builder()
            .project("simple/project")
            .id(10)
            .active(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
