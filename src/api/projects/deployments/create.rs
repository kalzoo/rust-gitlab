// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::projects::deployments::DeploymentStatus;

/// Create a deployment for a project.
#[derive(Debug, Builder, Clone)]
pub struct CreateDeployment<'a> {
    /// The project to edit a deployment from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The environment to create the deployment for.
    #[builder(setter(into))]
    environment: Cow<'a, str>,
    /// The SHA of the commit being deployed.
    #[builder(setter(into))]
    sha: Cow<'a, str>,
    /// The name of the branch or tag that is being deployed.
    #[builder(setter(into))]
    ref_: Cow<'a, str>,
    /// Whether `ref_` is a tag or branch.
    tag: bool,
    /// The status of the deployment.
    status: DeploymentStatus,
}

impl<'a> CreateDeployment<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateDeploymentBuilder<'a> {
        CreateDeploymentBuilder::default()
    }
}

impl<'a> Endpoint for CreateDeployment<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deployments", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("environment", &self.environment)
            .push("sha", &self.sha)
            .push("ref", &self.ref_)
            .push("tag", self.tag)
            .push("status", self.status);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::deployments::{
        CreateDeployment, CreateDeploymentBuilderError, DeploymentStatus,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn required_parameters() {
        let err = CreateDeployment::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateDeployment::builder()
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .tag(false)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "project");
    }

    #[test]
    fn environment_is_necessary() {
        let err = CreateDeployment::builder()
            .project("project")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .tag(false)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "environment");
    }

    #[test]
    fn sha_is_necessary() {
        let err = CreateDeployment::builder()
            .project("project")
            .environment("env")
            .ref_("main")
            .tag(false)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "sha");
    }

    #[test]
    fn ref_is_necessary() {
        let err = CreateDeployment::builder()
            .project("project")
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .tag(false)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "ref_");
    }

    #[test]
    fn tag_is_necessary() {
        let err = CreateDeployment::builder()
            .project("project")
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .status(DeploymentStatus::Success)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "tag");
    }

    #[test]
    fn status_is_necessary() {
        let err = CreateDeployment::builder()
            .project("project")
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .tag(false)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateDeploymentBuilderError, "status");
    }

    #[test]
    fn sufficient_parameters() {
        CreateDeployment::builder()
            .project("project")
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .tag(false)
            .status(DeploymentStatus::Success)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/deployments")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "environment=env",
                "&sha=0000000000000000000000000000000000000000",
                "&ref=main",
                "&tag=false",
                "&status=canceled",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateDeployment::builder()
            .project("simple/project")
            .environment("env")
            .sha("0000000000000000000000000000000000000000")
            .ref_("main")
            .tag(false)
            .status(DeploymentStatus::Canceled)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
