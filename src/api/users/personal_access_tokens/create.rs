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
pub enum PersonalAccessTokenCreateScope {
    /// Access to perform Kubernetes API calls.
    K8sProxy,
}

impl PersonalAccessTokenCreateScope {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::K8sProxy => "k8s_proxy",
        }
    }
}

impl ParamValue<'static> for PersonalAccessTokenCreateScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Create a new personal access token for the authenticated user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreatePersonalAccessToken<'a> {
    /// The name of the personal access token.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The scopes to allow the token to access.
    #[builder(setter(name = "_scopes"), private)]
    scopes: BTreeSet<PersonalAccessTokenCreateScope>,

    /// When the token expires.
    #[builder(default)]
    expires_at: Option<NaiveDate>,
}

impl<'a> CreatePersonalAccessToken<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePersonalAccessTokenBuilder<'a> {
        CreatePersonalAccessTokenBuilder::default()
    }
}

impl<'a> CreatePersonalAccessTokenBuilder<'a> {
    /// Add a scope for the token.
    pub fn scope(&mut self, scope: PersonalAccessTokenCreateScope) -> &mut Self {
        self.scopes.get_or_insert_with(BTreeSet::new).insert(scope);
        self
    }

    /// Add scopes for the token.
    pub fn scopes<I>(&mut self, scopes: I) -> &mut Self
    where
        I: Iterator<Item = PersonalAccessTokenCreateScope>,
    {
        self.scopes.get_or_insert_with(BTreeSet::new).extend(scopes);
        self
    }
}

impl<'a> Endpoint for CreatePersonalAccessToken<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users/personal_access_tokens".into()
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
        CreatePersonalAccessToken, CreatePersonalAccessTokenBuilderError,
        PersonalAccessTokenCreateScope,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn personal_access_token_create_scope_as_str() {
        let items = &[(PersonalAccessTokenCreateScope::K8sProxy, "k8s_proxy")];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn name_and_scopes_are_necessary() {
        let err = CreatePersonalAccessToken::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreatePersonalAccessTokenBuilderError, "name");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreatePersonalAccessToken::builder()
            .scope(PersonalAccessTokenCreateScope::K8sProxy)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePersonalAccessTokenBuilderError, "name");
    }

    #[test]
    fn scopes_is_necessary() {
        let err = CreatePersonalAccessToken::builder()
            .name("name")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreatePersonalAccessTokenBuilderError, "scopes");
    }

    #[test]
    fn user_name_and_scopes_are_sufficient() {
        CreatePersonalAccessToken::builder()
            .name("name")
            .scope(PersonalAccessTokenCreateScope::K8sProxy)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/personal_access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("name=name", "&scopes%5B%5D=k8s_proxy"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePersonalAccessToken::builder()
            .name("name")
            .scopes([PersonalAccessTokenCreateScope::K8sProxy].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_expires_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users/personal_access_tokens")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "name=name",
                "&expires_at=2022-01-01",
                "&scopes%5B%5D=k8s_proxy",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePersonalAccessToken::builder()
            .name("name")
            .scope(PersonalAccessTokenCreateScope::K8sProxy)
            .expires_at(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
