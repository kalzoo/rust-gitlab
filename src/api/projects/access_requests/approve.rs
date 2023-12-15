// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Access levels for projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ProjectAccessLevel {
    /// Guest access (can see the project).
    Guest,
    /// Reporter access (can open issues).
    Reporter,
    /// Developer access (can push branches, handle issues and merge requests).
    Developer,
    /// Maintainer access (can push to protected branches).
    Maintainer,
    /// Owner access (full rights).
    Owner,
}

impl ProjectAccessLevel {
    /// The string representation of the access level.
    pub fn as_str(self) -> &'static str {
        match self {
            ProjectAccessLevel::Owner => "owner",
            ProjectAccessLevel::Maintainer => "maintainer",
            ProjectAccessLevel::Developer => "developer",
            ProjectAccessLevel::Reporter => "reporter",
            ProjectAccessLevel::Guest => "guest",
        }
    }

    /// The integer representation of the access level.
    pub fn as_u64(self) -> u64 {
        match self {
            ProjectAccessLevel::Owner => 50,
            ProjectAccessLevel::Maintainer => 40,
            ProjectAccessLevel::Developer => 30,
            ProjectAccessLevel::Reporter => 20,
            ProjectAccessLevel::Guest => 10,
        }
    }
}

/// Submit approval for a user access request to a project
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectAccessRequestsApprove<'a> {
    /// The project to query for membership.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The user ID of the access requester
    user_id: u64,

    /// A valid access level (defaults: the Developer role)
    #[builder(default)]
    access_level: Option<ProjectAccessLevel>,
}

impl<'a> ProjectAccessRequestsApprove<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectAccessRequestsApproveBuilder<'a> {
        ProjectAccessRequestsApproveBuilder::default()
    }
}

impl<'a> Endpoint for ProjectAccessRequestsApprove<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/access_requests/{}/approve",
            self.project, self.user_id,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params.push_opt(
            "access_level",
            self.access_level.map(|level| level.as_u64()),
        );

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::common::AccessLevel;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use crate::api::projects::access_requests::{
        ProjectAccessLevel, ProjectAccessRequestsApprove, ProjectAccessRequestsApproveBuilderError,
    };

    use http::Method;

    #[test]
    fn common_access_level_consisent() {
        let items = &[
            (ProjectAccessLevel::Guest, AccessLevel::Guest),
            (ProjectAccessLevel::Reporter, AccessLevel::Reporter),
            (ProjectAccessLevel::Developer, AccessLevel::Developer),
            (ProjectAccessLevel::Maintainer, AccessLevel::Maintainer),
            (ProjectAccessLevel::Owner, AccessLevel::Owner),
        ];

        for (g, c) in items {
            assert_eq!(g.as_str(), c.as_str());
            assert_eq!(g.as_u64(), c.as_u64());
        }
    }

    #[test]
    fn access_level_as_str() {
        let items = &[
            (ProjectAccessLevel::Guest, "guest", 10),
            (ProjectAccessLevel::Reporter, "reporter", 20),
            (ProjectAccessLevel::Developer, "developer", 30),
            (ProjectAccessLevel::Maintainer, "maintainer", 40),
            (ProjectAccessLevel::Owner, "owner", 50),
        ];

        for (i, s, u) in items {
            assert_eq!(i.as_str(), *s);
            assert_eq!(i.as_u64(), *u);
        }
    }

    #[test]
    fn access_level_ordering() {
        let items = &[
            ProjectAccessLevel::Guest,
            ProjectAccessLevel::Reporter,
            ProjectAccessLevel::Developer,
            ProjectAccessLevel::Maintainer,
            ProjectAccessLevel::Owner,
        ];

        let mut last = None;
        for item in items {
            if let Some(prev) = last {
                assert!(prev < item);
            }
            last = Some(item);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = ProjectAccessRequestsApprove::builder()
            .user_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            ProjectAccessRequestsApproveBuilderError,
            "project"
        );
    }

    #[test]
    fn user_id_is_needed() {
        let err = ProjectAccessRequestsApprove::builder()
            .project(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            ProjectAccessRequestsApproveBuilderError,
            "user_id"
        );
    }

    #[test]
    fn user_project_is_sufficient() {
        ProjectAccessRequestsApprove::builder()
            .project(1)
            .user_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/access_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessRequestsApprove::builder()
            .project("simple/project")
            .user_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/simple%2Fproject/access_requests/1/approve")
            .content_type("application/x-www-form-urlencoded")
            .body_str("access_level=30")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectAccessRequestsApprove::builder()
            .project("simple/project")
            .user_id(1)
            .access_level(ProjectAccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
