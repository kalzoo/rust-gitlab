// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Take ownership of a pipeline schedule on a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct TakePipelineScheduleOwnership<'a> {
    /// The project to take ownership of a pipeline schedule on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The pipeline schedule id.
    id: u64,
}

impl<'a> TakePipelineScheduleOwnership<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TakePipelineScheduleOwnershipBuilder<'a> {
        TakePipelineScheduleOwnershipBuilder::default()
    }
}

impl<'a> Endpoint for TakePipelineScheduleOwnership<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/pipeline_schedules/{}/take_ownership",
            self.project, self.id
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::pipeline_schedules::{
        TakePipelineScheduleOwnership, TakePipelineScheduleOwnershipBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_id_are_needed() {
        let err = TakePipelineScheduleOwnership::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            TakePipelineScheduleOwnershipBuilderError,
            "project"
        );
    }

    #[test]
    fn project_is_needed() {
        let err = TakePipelineScheduleOwnership::builder()
            .id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            TakePipelineScheduleOwnershipBuilderError,
            "project"
        );
    }

    #[test]
    fn id_is_needed() {
        let err = TakePipelineScheduleOwnership::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, TakePipelineScheduleOwnershipBuilderError, "id");
    }

    #[test]
    fn project_and_id_are_sufficient() {
        TakePipelineScheduleOwnership::builder()
            .project(1)
            .id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/pipeline_schedules/10/take_ownership")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = TakePipelineScheduleOwnership::builder()
            .project("simple/project")
            .id(10)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
