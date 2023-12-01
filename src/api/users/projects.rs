// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! User projects API endpoints.
//!
//! These endpoints are used for querying user projects.

use derive_builder::Builder;

use crate::api::common::{AccessLevel, NameOrId, SortOrder, VisibilityLevel};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys project results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserProjectsOrderBy {
    /// Order by the user ID.
    Id,
    /// Order by the user display name.
    Name,
    /// Order by the path.
    Path,
    /// Order by the creation date of the project.
    CreatedAt,
    /// Order by the last updated date of the project.
    UpdatedAt,
    /// Order by a similarity score based on the search.
    Similarity,
    /// Order by the last activity date of the project.
    LastActivityAt,
}

#[allow(clippy::derivable_impls)]
impl Default for UserProjectsOrderBy {
    fn default() -> Self {
        // XXX(rust-1.62): use `#[default]`
        UserProjectsOrderBy::CreatedAt
    }
}

impl UserProjectsOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            UserProjectsOrderBy::Id => "id",
            UserProjectsOrderBy::Name => "name",
            UserProjectsOrderBy::Path => "path",
            UserProjectsOrderBy::CreatedAt => "created_at",
            UserProjectsOrderBy::UpdatedAt => "updated_at",
            UserProjectsOrderBy::Similarity => "similarity",
            UserProjectsOrderBy::LastActivityAt => "last_activity_at",
        }
    }
}

impl ParamValue<'static> for UserProjectsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query projects of a user.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct UserProjects<'a> {
    /// The user to query for projects.
    #[builder(setter(into))]
    user: NameOrId<'a>,

    /// Limit by archived status.
    #[builder(default)]
    archived: Option<bool>,
    /// Limit by visibility public, internal, or private
    #[builder(default)]
    visibility: Option<VisibilityLevel>,

    /// Return projects ordered by keys.
    #[builder(default)]
    order_by: Option<UserProjectsOrderBy>,
    /// Return projects sorted in asc or desc order.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Search for projects using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,

    /// Return only the ID, URL, name, and path of each project.
    #[builder(default)]
    simple: Option<bool>,
    /// Limit by projects owned by the current user.
    #[builder(default)]
    owned: Option<bool>,
    /// Limit by projects starred by the current user.
    #[builder(default)]
    starred: Option<bool>,
    /// Limit by projects with issues feature enabled.
    #[builder(default)]
    with_issues_enabled: Option<bool>,
    /// Limit by projects with merge requests feature enabled.
    #[builder(default)]
    with_merge_requests_enabled: Option<bool>,
    /// Limit to projects where current user has at least this access level.
    #[builder(default)]
    min_access_level: Option<AccessLevel>,
    /// Include custom attributes in response (admins only).
    #[builder(default)]
    with_custom_attributes: Option<bool>,
}

impl<'a> UserProjects<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UserProjectsBuilder<'a> {
        UserProjectsBuilder::default()
    }
}

impl<'a> Endpoint for UserProjects<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/projects", self.user).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("archived", self.archived)
            .push_opt("visibility", self.visibility)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("search", self.search.as_ref())
            .push_opt("simple", self.simple)
            .push_opt("owned", self.owned)
            .push_opt("starred", self.starred)
            .push_opt("with_issues_enabled", self.with_issues_enabled)
            .push_opt(
                "with_merge_requests_enabled",
                self.with_merge_requests_enabled,
            )
            .push_opt(
                "min_access_level",
                self.min_access_level.map(AccessLevel::as_u64),
            )
            .push_opt("with_custom_attributes", self.with_custom_attributes);

        params
    }
}

impl<'a> Pageable for UserProjects<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::{AccessLevel, SortOrder, VisibilityLevel};
    use crate::api::users::projects::{
        UserProjects, UserProjectsBuilderError, UserProjectsOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(
            UserProjectsOrderBy::default(),
            UserProjectsOrderBy::CreatedAt
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (UserProjectsOrderBy::Id, "id"),
            (UserProjectsOrderBy::Name, "name"),
            (UserProjectsOrderBy::Path, "path"),
            (UserProjectsOrderBy::CreatedAt, "created_at"),
            (UserProjectsOrderBy::UpdatedAt, "updated_at"),
            (UserProjectsOrderBy::Similarity, "similarity"),
            (UserProjectsOrderBy::LastActivityAt, "last_activity_at"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn user_is_needed() {
        let err = UserProjects::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UserProjectsBuilderError, "user");
    }

    #[test]
    fn user_is_sufficient() {
        UserProjects::builder().user(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder().user("user").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_archived() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("archived", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .archived(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_visibility() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("visibility", "private")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .visibility(VisibilityLevel::Private)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .order_by(UserProjectsOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("search", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .search("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_simple() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("simple", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .simple(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_owned() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("owned", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .owned(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_starred() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("starred", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .starred(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_issues_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("with_issues_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .with_issues_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_merge_requests_enabled() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("with_merge_requests_enabled", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .with_merge_requests_enabled(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_min_access_level() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("min_access_level", "30")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .min_access_level(AccessLevel::Developer)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_custom_attributes() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/user/projects")
            .add_query_params(&[("with_custom_attributes", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UserProjects::builder()
            .user("user")
            .with_custom_attributes(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
