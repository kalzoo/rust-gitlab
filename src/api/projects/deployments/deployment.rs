// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Get a deployment for a project.
#[derive(Debug, Builder, Clone)]
pub struct Deployment<'a> {
    /// The project to get a deployment from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the deployment.
    deployment_id: u64,
}

impl<'a> Deployment<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeploymentBuilder<'a> {
        DeploymentBuilder::default()
    }
}

impl<'a> Endpoint for Deployment<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/deployments/{}",
            self.project, self.deployment_id,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deployments::{Deployment, DeploymentBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deployment_id_are_necessary() {
        let err = Deployment::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeploymentBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = Deployment::builder().deployment_id(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeploymentBuilderError, "project");
    }

    #[test]
    fn deployment_id_is_necessary() {
        let err = Deployment::builder()
            .project("project")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeploymentBuilderError, "deployment_id");
    }

    #[test]
    fn project_and_deployment_id_are_sufficient() {
        Deployment::builder()
            .project("project")
            .deployment_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/deployments/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployment::builder()
            .project("simple/project")
            .deployment_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
