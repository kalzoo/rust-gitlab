// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Scopes for personal access tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ProjectAccessTokenScope {
    /// Access the API and perform git reads and writes.
    Api,
    /// Access read-only API endpoints.
    ReadApi,
    /// Read access to repositories.
    ReadRepository,
    /// Write access to repositories.
    WriteRepository,
    /// Read access to Docker registries.
    ReadRegistry,
    /// Write access to Docker registries.
    WriteRegistry,
    /// Permission to create instance runners.
    CreateRunner,
    /// Access to AI features (GitLab Duo for JetBrains).
    AiFeatures,
    /// Access to perform Kubernetes API calls.
    K8sProxy,
}

impl ProjectAccessTokenScope {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::ReadApi => "read_api",
            Self::ReadRepository => "read_repository",
            Self::WriteRepository => "write_repository",
            Self::ReadRegistry => "read_registry",
            Self::WriteRegistry => "write_registry",
            Self::CreateRunner => "create_runner",
            Self::AiFeatures => "ai_features",
            Self::K8sProxy => "k8s_proxy",
        }
    }
}

impl ParamValue<'static> for ProjectAccessTokenScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Access levels for groups and projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ProjectAccessTokenAccessLevel {
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

impl From<ProjectAccessTokenAccessLevel> for AccessLevel {
    fn from(p: ProjectAccessTokenAccessLevel) -> Self {
        match p {
            ProjectAccessTokenAccessLevel::Guest => Self::Guest,
            ProjectAccessTokenAccessLevel::Reporter => Self::Reporter,
            ProjectAccessTokenAccessLevel::Developer => Self::Developer,
            ProjectAccessTokenAccessLevel::Maintainer => Self::Maintainer,
            ProjectAccessTokenAccessLevel::Owner => Self::Owner,
        }
    }
}

/// Create a new personal access token for the authenticated user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateProjectAccessToken<'a> {
    /// The project to create the access token for.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the personal access token.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The scopes to allow the token to access.
    #[builder(setter(name = "_scopes"), private)]
    scopes: BTreeSet<ProjectAccessTokenScope>,

    /// When the token expires.
    #[builder(default)]
    access_level: Option<ProjectAccessTokenAccessLevel>,
    /// When the token expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> CreateProjectAccessToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateProjectAccessTokenBuilder<'a> {
        CreateProjectAccessTokenBuilder::default()
    }
}

impl<'a> CreateProjectAccessTokenBuilder<'a> {
    /// Add a scope for the token.
    pub fn scope(&mut self, scope: ProjectAccessTokenScope) -> &mut Self {
        self.scopes.get_or_insert_with(BTreeSet::new).insert(scope);
        self
    }

    /// Add scopes for the token.
    pub fn scopes<I>(&mut self, scopes: I) -> &mut Self
    where
        I: Iterator<Item = ProjectAccessTokenScope>,
    {
        self.scopes.get_or_insert_with(BTreeSet::new).extend(scopes);
        self
    }
}

impl<'a> Endpoint for CreateProjectAccessToken<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/access_tokens", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push_opt(
                "access_level",
                self.access_level.map(|a| AccessLevel::from(a).as_u64()),
            )
            .push_opt("expires_at", self.expires_at);

        params.extend(self.scopes.iter().map(|&value| ("scopes[]", value)));

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::projects::access_tokens::{
        CreateProjectAccessToken, CreateProjectAccessTokenBuilderError,
        ProjectAccessTokenAccessLevel, ProjectAccessTokenScope,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn personal_access_token_create_scope_as_str() {
        let items = &[
            (ProjectAccessTokenScope::Api, "api"),
            (ProjectAccessTokenScope::ReadApi, "read_api"),
            (ProjectAccessTokenScope::ReadRepository, "read_repository"),
            (ProjectAccessTokenScope::WriteRepository, "write_repository"),
            (ProjectAccessTokenScope::ReadRegistry, "read_registry"),
            (ProjectAccessTokenScope::WriteRegistry, "write_registry"),
            (ProjectAccessTokenScope::CreateRunner, "create_runner"),
            (ProjectAccessTokenScope::AiFeatures, "ai_features"),
            (ProjectAccessTokenScope::K8sProxy, "k8s_proxy"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_token_access_level_from() {
        use crate::api::common::AccessLevel;

        let items = &[
            (ProjectAccessTokenAccessLevel::Guest, AccessLevel::Guest),
            (
                ProjectAccessTokenAccessLevel::Reporter,
                AccessLevel::Reporter,
            ),
            (
                ProjectAccessTokenAccessLevel::Developer,
                AccessLevel::Developer,
            ),
            (
                ProjectAccessTokenAccessLevel::Maintainer,
                AccessLevel::Maintainer,
            ),
            (ProjectAccessTokenAccessLevel::Owner, AccessLevel::Owner),
        ];

        for (i, s) in items {
            assert_eq!(AccessLevel::from(*i), *s);
        }
    }

    #[test]
    fn project_name_and_scopes_are_necessary() {
        let err = CreateProjectAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectAccessTokenBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateProjectAccessToken::builder()
            .name("name")
            .scope(ProjectAccessTokenScope::K8sProxy)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectAccessTokenBuilderError, "project");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateProjectAccessToken::builder()
            .project(1)
            .scope(ProjectAccessTokenScope::K8sProxy)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectAccessTokenBuilderError, "name");
    }

    #[test]
    fn scopes_is_necessary() {
        let err = CreateProjectAccessToken::builder()
            .project(1)
            .name("name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateProjectAccessTokenBuilderError, "scopes");
    }

    #[test]
    fn project_name_and_scopes_are_sufficient() {
        CreateProjectAccessToken::builder()
            .project(1)
            .name("name")
            .scope(ProjectAccessTokenScope::K8sProxy)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1/access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&scopes%5B%5D=k8s_proxy"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectAccessToken::builder()
            .project(1)
            .name("name")
            .scopes([ProjectAccessTokenScope::K8sProxy].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1/access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&access_level=30",
                "&scopes%5B%5D=k8s_proxy",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectAccessToken::builder()
            .project(1)
            .name("name")
            .scope(ProjectAccessTokenScope::K8sProxy)
            .access_level(ProjectAccessTokenAccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1/access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&expires_at=2022-01-01",
                "&scopes%5B%5D=k8s_proxy",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateProjectAccessToken::builder()
            .project(1)
            .name("name")
            .scope(ProjectAccessTokenScope::K8sProxy)
            .expires_at(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
