// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Get assets as links from a release.
#[derive(Debug, Builder, Clone)]
pub struct ListReleaseLinks<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The tag associated with the Release.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,
}

impl<'a> ListReleaseLinks<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ListReleaseLinksBuilder<'a> {
        ListReleaseLinksBuilder::default()
    }
}

impl<'a> Endpoint for ListReleaseLinks<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/releases/{}/assets/links",
            self.project,
            common::path_escaped(self.tag_name.as_ref()),
        )
        .into()
    }
}

impl<'a> Pageable for ListReleaseLinks<'a> {}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{self, projects::releases::links::ListReleaseLinksBuilderError, Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::ListReleaseLinks;

    #[test]
    fn project_is_needed() {
        let err = ListReleaseLinks::builder()
            .tag_name("1.2.3")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, ListReleaseLinksBuilderError, "project");
    }

    #[test]
    fn tag_name_is_needed() {
        let err = ListReleaseLinks::builder().project(1).build().unwrap_err();

        crate::test::assert_missing_field!(err, ListReleaseLinksBuilderError, "tag_name");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        ListReleaseLinks::builder()
            .project(1)
            .tag_name("1.2.3")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/1337/releases/1.2.3%2001/assets/links")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ListReleaseLinks::builder()
            .project(1337)
            .tag_name("1.2.3 01")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
