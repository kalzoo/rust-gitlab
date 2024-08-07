// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use chrono::NaiveDate;
use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Scopes for personal access tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum PersonalAccessTokenScope {
    /// Access the API and perform git reads and writes.
    Api,
    /// Access to read the user information.
    ReadUser,
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
    /// Permission to `sudo` as other users (administrator only).
    Sudo,
    /// Permission to access administrator API actions.
    AdminMode,
    /// Permission to create instance runners.
    CreateRunner,
    /// Access to AI features (GitLab Duo for JetBrains).
    AiFeatures,
    /// Access to perform Kubernetes API calls.
    K8sProxy,
    /// Access to the Service Ping payload.
    ReadServicePing,
}

impl PersonalAccessTokenScope {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::ReadUser => "read_user",
            Self::ReadApi => "read_api",
            Self::ReadRepository => "read_repository",
            Self::WriteRepository => "write_repository",
            Self::ReadRegistry => "read_registry",
            Self::WriteRegistry => "write_registry",
            Self::Sudo => "sudo",
            Self::AdminMode => "admin_mode",
            Self::CreateRunner => "create_runner",
            Self::AiFeatures => "ai_features",
            Self::K8sProxy => "k8s_proxy",
            Self::ReadServicePing => "read_service_ping",
        }
    }

    /// Transform into a `create` scope if possible.
    pub fn as_create_scope(self) -> Option<super::PersonalAccessTokenCreateScope> {
        match self {
            Self::K8sProxy => Some(super::PersonalAccessTokenCreateScope::K8sProxy),
            _ => None,
        }
    }
}

impl ParamValue<'static> for PersonalAccessTokenScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Create a new personal access token for a user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreatePersonalAccessTokenForUser<'a> {
    /// The user to create a personal access token for.
    user: u64,
    /// The name of the personal access token.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The scopes to allow the token to access.
    #[builder(setter(name = "_scopes"), private)]
    scopes: BTreeSet<PersonalAccessTokenScope>,

    /// When the token expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> CreatePersonalAccessTokenForUser<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePersonalAccessTokenForUserBuilder<'a> {
        CreatePersonalAccessTokenForUserBuilder::default()
    }
}

impl<'a> CreatePersonalAccessTokenForUserBuilder<'a> {
    /// Add a scope for the token.
    pub fn scope(&mut self, scope: PersonalAccessTokenScope) -> &mut Self {
        self.scopes.get_or_insert_with(BTreeSet::new).insert(scope);
        self
    }

    /// Add scopes for the token.
    pub fn scopes<I>(&mut self, scopes: I) -> &mut Self
    where
        I: Iterator<Item = PersonalAccessTokenScope>,
    {
        self.scopes.get_or_insert_with(BTreeSet::new).extend(scopes);
        self
    }
}

impl<'a> Endpoint for CreatePersonalAccessTokenForUser<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/personal_access_tokens", self.user).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push_opt("expires_at", self.expires_at);

        params.extend(self.scopes.iter().map(|&value| ("scopes[]", value)));

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Method;

    use crate::api::users::personal_access_tokens::{
        CreatePersonalAccessTokenForUser, CreatePersonalAccessTokenForUserBuilderError,
        PersonalAccessTokenScope,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn personal_access_token_scope_as_str() {
        let items = &[
            (PersonalAccessTokenScope::Api, "api"),
            (PersonalAccessTokenScope::ReadUser, "read_user"),
            (PersonalAccessTokenScope::ReadApi, "read_api"),
            (PersonalAccessTokenScope::ReadRepository, "read_repository"),
            (
                PersonalAccessTokenScope::WriteRepository,
                "write_repository",
            ),
            (PersonalAccessTokenScope::ReadRegistry, "read_registry"),
            (PersonalAccessTokenScope::WriteRegistry, "write_registry"),
            (PersonalAccessTokenScope::Sudo, "sudo"),
            (PersonalAccessTokenScope::AdminMode, "admin_mode"),
            (PersonalAccessTokenScope::CreateRunner, "create_runner"),
            (PersonalAccessTokenScope::AiFeatures, "ai_features"),
            (PersonalAccessTokenScope::K8sProxy, "k8s_proxy"),
            (
                PersonalAccessTokenScope::ReadServicePing,
                "read_service_ping",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn user_name_and_scopes_are_necessary() {
        let err = CreatePersonalAccessTokenForUser::builder()
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePersonalAccessTokenForUserBuilderError,
            "user",
        );
    }

    #[test]
    fn user_is_necessary() {
        let err = CreatePersonalAccessTokenForUser::builder()
            .name("name")
            .scope(PersonalAccessTokenScope::Api)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePersonalAccessTokenForUserBuilderError,
            "user",
        );
    }

    #[test]
    fn name_is_necessary() {
        let err = CreatePersonalAccessTokenForUser::builder()
            .user(1)
            .scope(PersonalAccessTokenScope::Api)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePersonalAccessTokenForUserBuilderError,
            "name",
        );
    }

    #[test]
    fn scopes_is_necessary() {
        let err = CreatePersonalAccessTokenForUser::builder()
            .user(1)
            .name("name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(
            err,
            CreatePersonalAccessTokenForUserBuilderError,
            "scopes",
        );
    }

    #[test]
    fn user_name_and_scopes_are_sufficient() {
        CreatePersonalAccessTokenForUser::builder()
            .user(1)
            .name("name")
            .scope(PersonalAccessTokenScope::ReadUser)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/1/personal_access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&scopes%5B%5D=api"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePersonalAccessTokenForUser::builder()
            .user(1)
            .name("name")
            .scopes([PersonalAccessTokenScope::Api].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/1/personal_access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&expires_at=2022-01-01",
                "&scopes%5B%5D=api",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePersonalAccessTokenForUser::builder()
            .user(1)
            .name("name")
            .scope(PersonalAccessTokenScope::Api)
            .expires_at(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
