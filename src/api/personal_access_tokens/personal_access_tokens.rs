// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// States of personal access tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PersonalAccessTokenState {
    /// Filter to return only active tokens.
    Active,
    /// Filter to return only inactive tokens.
    Inactive,
}

impl PersonalAccessTokenState {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            PersonalAccessTokenState::Active => "active",
            PersonalAccessTokenState::Inactive => "inactive",
        }
    }
}

impl ParamValue<'static> for PersonalAccessTokenState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Get personal access tokens of the authenticated user.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PersonalAccessTokens<'a> {
    /// Limit results to personal access tokens created after a time.
    #[builder(default)]
    created_after: Option<DateTime<Utc>>,
    /// Limit results to personal access tokens created before a time.
    #[builder(default)]
    created_before: Option<DateTime<Utc>>,
    /// Limit results to personal access tokens last used after a time.
    #[builder(default)]
    last_used_after: Option<DateTime<Utc>>,
    /// Limit results to personal access tokens last used before a time.
    #[builder(default)]
    last_used_before: Option<DateTime<Utc>>,

    /// Limit results to personal access tokens with the given revocation state.
    #[builder(default)]
    revoked: Option<bool>,
    /// Search for personal access tokens with names containing a string.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Filter based on state.
    #[builder(default)]
    state: Option<PersonalAccessTokenState>,

    /// Filter based on the owning user.
    #[builder(setter(into), default)]
    user: Option<NameOrId<'a>>,
}

impl<'a> PersonalAccessTokens<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PersonalAccessTokensBuilder<'a> {
        PersonalAccessTokensBuilder::default()
    }
}

impl<'a> Endpoint for PersonalAccessTokens<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "personal_access_tokens".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("created_after", self.created_after)
            .push_opt("created_before", self.created_before)
            .push_opt("last_used_after", self.last_used_after)
            .push_opt("last_used_before", self.last_used_before)
            .push_opt("revoked", self.revoked)
            .push_opt("search", self.search.as_ref())
            .push_opt("state", self.state)
            .push_opt("user_id", self.user.as_ref());

        params
    }
}

impl<'a> Pageable for PersonalAccessTokens<'a> {}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use http::Method;

    use crate::api::personal_access_tokens::{PersonalAccessTokenState, PersonalAccessTokens};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn personal_access_token_state_as_str() {
        let items = &[
            (PersonalAccessTokenState::Active, "active"),
            (PersonalAccessTokenState::Inactive, "inactive"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn default_is_sufficient() {
        PersonalAccessTokens::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_after() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("created_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .created_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_before() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("created_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .created_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_used_after() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("last_used_after", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .last_used_after(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_last_used_before() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("last_used_before", "2020-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .last_used_before(Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_revoked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("revoked", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .revoked(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("search", "needle")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .search("needle")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("state", "active")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder()
            .state(PersonalAccessTokenState::Active)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_user() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("personal_access_tokens")
            .add_query_params(&[("user_id", "100")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PersonalAccessTokens::builder().user(100).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
