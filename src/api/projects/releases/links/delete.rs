// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Delete an asset as a link from a release.
#[derive(Debug, Builder, Clone)]
pub struct DeleteReleaseLink<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The tag associated with the Release.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,

    /// The id of the link.
    #[builder(setter(into))]
    link_id: NameOrId<'a>,
}

impl<'a> DeleteReleaseLink<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteReleaseLinkBuilder<'a> {
        DeleteReleaseLinkBuilder::default()
    }
}

impl<'a> Endpoint for DeleteReleaseLink<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/releases/{}/assets/links/{}",
            self.project,
            common::path_escaped(self.tag_name.as_ref()),
            self.link_id,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{self, projects::releases::links::DeleteReleaseLinkBuilderError, Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::DeleteReleaseLink;

    #[test]
    fn project_is_needed() {
        let err = DeleteReleaseLink::builder()
            .tag_name("1.2.3")
            .link_id(123)
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, DeleteReleaseLinkBuilderError, "project");
    }

    #[test]
    fn tag_name_is_needed() {
        let err = DeleteReleaseLink::builder()
            .project(1)
            .link_id(123)
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, DeleteReleaseLinkBuilderError, "tag_name");
    }

    #[test]
    fn link_id_is_needed() {
        let err = DeleteReleaseLink::builder()
            .tag_name("1.2.3")
            .project(1)
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, DeleteReleaseLinkBuilderError, "link_id");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        DeleteReleaseLink::builder()
            .project(1)
            .tag_name("1.2.3")
            .link_id(123)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/1337/releases/1.2.3%2001/assets/links/123")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteReleaseLink::builder()
            .project(1337)
            .tag_name("1.2.3 01")
            .link_id(123)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
