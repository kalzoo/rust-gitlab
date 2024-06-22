// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a deployment for a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteDeployment<'a> {
    /// The project to delete a deployment from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the deployment to delete.
    deployment_id: u64,
}

impl<'a> DeleteDeployment<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteDeploymentBuilder<'a> {
        DeleteDeploymentBuilder::default()
    }
}

impl<'a> Endpoint for DeleteDeployment<'a> {
    fn method(&self) -> Method {
        Method::DELETE
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

    use crate::api::projects::deployments::{DeleteDeployment, DeleteDeploymentBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_deployment_id_are_necessary() {
        let err = DeleteDeployment::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeploymentBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = DeleteDeployment::builder()
            .deployment_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeploymentBuilderError, "project");
    }

    #[test]
    fn deployment_id_is_necessary() {
        let err = DeleteDeployment::builder()
            .project("project")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeleteDeploymentBuilderError, "deployment_id");
    }

    #[test]
    fn project_and_deployment_id_are_sufficient() {
        DeleteDeployment::builder()
            .project("project")
            .deployment_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/deployments/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteDeployment::builder()
            .project("simple/project")
            .deployment_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
