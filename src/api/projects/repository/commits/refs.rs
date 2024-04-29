// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::{endpoint_prelude::*, ParamValue};

/// Commit reference types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommitRefsType {
    /// Scope branch
    Branch,
    /// Scope tag
    Tag,
    /// Scope all
    All,
}

impl CommitRefsType {
    fn as_str(self) -> &'static str {
        match self {
            CommitRefsType::Branch => "branch",
            CommitRefsType::Tag => "tag",
            CommitRefsType::All => "all",
        }
    }
}

impl ParamValue<'static> for CommitRefsType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Post a comment on a specific commit in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CommitReferences<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit sha
    #[builder(setter(into))]
    sha: Cow<'a, str>,
    /// The ref types
    #[builder(default)]
    type_: Option<CommitRefsType>,
}

impl<'a> CommitReferences<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitReferencesBuilder<'a> {
        CommitReferencesBuilder::default()
    }
}

impl<'a> Endpoint for CommitReferences<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/refs",
            self.project, self.sha,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("type", self.type_);

        params
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::commits::refs::{
        CommitReferences, CommitReferencesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use super::CommitRefsType;

    #[test]
    fn commit_refs_type_as_str() {
        let items = &[
            (CommitRefsType::Branch, "branch"),
            (CommitRefsType::Tag, "tag"),
            (CommitRefsType::All, "all"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_necessary() {
        let err = CommitReferences::builder()
            .sha("0000000000000000000000000000000000000000")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CommitReferencesBuilderError, "project");
    }

    #[test]
    fn sha_is_necessary() {
        let err = CommitReferences::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitReferencesBuilderError, "sha");
    }

    #[test]
    fn project_and_sha_are_sufficient() {
        CommitReferences::builder()
            .project(1)
            .sha("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/refs")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitReferences::builder()
            .project("simple/project")
            .sha("0000000000000000000000000000000000000000")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/commits/0000000000000000000000000000000000000000/refs")
            .add_query_params(&[("type", "all")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CommitReferences::builder()
            .project("simple/project")
            .sha("0000000000000000000000000000000000000000")
            .type_(CommitRefsType::All)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
