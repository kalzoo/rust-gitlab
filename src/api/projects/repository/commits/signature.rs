// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Query for a specific signature in a project.
#[derive(Debug, Builder, Clone)]
pub struct Signature<'a> {
    /// The project to get a commit from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The commit to get the signature of
    #[builder(setter(into))]
    commit: Cow<'a, str>,
}

impl<'a> Signature<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> SignatureBuilder<'a> {
        SignatureBuilder::default()
    }
}

impl<'a> Endpoint for Signature<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/commits/{}/signature",
            self.project,
            common::path_escaped(&self.commit),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::commits::signature::{Signature, SignatureBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_commit_are_necessary() {
        let err = Signature::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, SignatureBuilderError, "project");
    }

    #[test]
    fn project_is_necessary() {
        let err = Signature::builder().commit("master").build().unwrap_err();
        crate::test::assert_missing_field!(err, SignatureBuilderError, "project");
    }

    #[test]
    fn commit_is_necessary() {
        let err = Signature::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, SignatureBuilderError, "commit");
    }

    #[test]
    fn project_and_commit_are_sufficient() {
        Signature::builder()
            .project(1)
            .commit("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits/nested%2Ftag/signature")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Signature::builder()
            .project("simple/project")
            .commit("nested/tag")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
