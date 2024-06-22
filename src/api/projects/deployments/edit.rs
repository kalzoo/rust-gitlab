// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// The status of a deployment.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DeploymentStatus {
    /// The deployment is running.
    Running,
    /// The deployment succeeded.
    Success,
    /// The deployment failed.
    Failed,
    /// The deployment is canceled.
    Canceled,
}

impl DeploymentStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
        }
    }
}

impl ParamValue<'static> for DeploymentStatus {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Edit a deployment for a project.
#[derive(Debug, Builder, Clone)]
pub struct EditDeployment<'a> {
    /// The project to edit a deployment from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the deployment to edit.
    deployment_id: u64,
    /// The status of the deployment.
    status: DeploymentStatus,
}

impl<'a> EditDeployment<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EditDeploymentBuilder<'a> {
        EditDeploymentBuilder::default()
    }
}

impl<'a> Endpoint for EditDeployment<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/deployments/{}",
            self.project, self.deployment_id,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push("status", self.status);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deployments::{
        DeploymentStatus, EditDeployment, EditDeploymentBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn deployment_status_as_str() {
        let items = &[
            (DeploymentStatus::Running, "running"),
            (DeploymentStatus::Success, "success"),
            (DeploymentStatus::Failed, "failed"),
            (DeploymentStatus::Canceled, "canceled"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_deployment_id_and_status_are_necessary() {
        let err = EditDeployment::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EditDeploymentBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = EditDeployment::builder()
            .deployment_id(1)
            .status(DeploymentStatus::Failed)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditDeploymentBuilderError, "project");
    }

    #[test]
    fn deployment_id_is_necessary() {
        let err = EditDeployment::builder()
            .project("project")
            .status(DeploymentStatus::Failed)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditDeploymentBuilderError, "deployment_id");
    }

    #[test]
    fn status_is_necessary() {
        let err = EditDeployment::builder()
            .project("project")
            .deployment_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, EditDeploymentBuilderError, "status");
    }

    #[test]
    fn sufficient_parameters() {
        EditDeployment::builder()
            .project("project")
            .deployment_id(1)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/deployments/1")
            .content_type("application/x-www-form-urlencoded")
            .body_str("status=canceled")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = EditDeployment::builder()
            .project("simple/project")
            .deployment_id(1)
            .status(DeploymentStatus::Canceled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
