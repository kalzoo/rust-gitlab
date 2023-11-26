// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

use super::LinkType;

/// Create a link to an asset for a release.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateReleaseLink<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The tag associated with the release.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,

    /// The name of the link.
    ///
    /// Link names must be unique in the release.
    #[builder(setter(into))]
    name: Cow<'a, str>,

    /// The URL of the link.
    ///
    /// Link URLs must be unique in the release.
    #[builder(setter(into))]
    url: Cow<'a, str>,

    /// Optional path for a direct asset link.
    #[builder(setter(into), default)]
    direct_asset_path: Option<Cow<'a, str>>,

    /// The type of the link: other, runbook, image, package.
    ///
    /// Defaults to other.
    #[builder(setter(into), default)]
    link_type: Option<LinkType>,
}

impl<'a> CreateReleaseLink<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateReleaseLinkBuilder<'a> {
        CreateReleaseLinkBuilder::default()
    }
}

impl<'a> Endpoint for CreateReleaseLink<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/releases/{}/assets/links",
            self.project,
            common::path_escaped(self.tag_name.as_ref()),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("name", &self.name)
            .push("url", &self.url)
            .push_opt("direct_asset_path", self.direct_asset_path.as_ref())
            .push_opt("link_type", self.link_type);

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{
            self,
            projects::releases::links::{CreateReleaseLinkBuilderError, LinkType},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::CreateReleaseLink;

    #[test]
    fn project_is_needed() {
        let err = CreateReleaseLink::builder()
            .tag_name("1.2.3")
            .name("test")
            .url("test.com/file")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseLinkBuilderError, "project");
    }

    #[test]
    fn tag_name_is_needed() {
        let err = CreateReleaseLink::builder()
            .project(1)
            .name("test")
            .url("test.com/file")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseLinkBuilderError, "tag_name");
    }

    #[test]
    fn name_is_needed() {
        let err = CreateReleaseLink::builder()
            .project(1)
            .tag_name("1.2.3")
            .url("test.com/file")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseLinkBuilderError, "name");
    }

    #[test]
    fn url_is_needed() {
        let err = CreateReleaseLink::builder()
            .project(1)
            .tag_name("1.2.3")
            .name("test")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseLinkBuilderError, "url");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        CreateReleaseLink::builder()
            .project(1)
            .tag_name("1.2.3")
            .name("test")
            .url("test.com/file")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases/1.2.3%2001/assets/links")
            .body_str(concat!("name=test", "&url=test.com%2Ffile"))
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateReleaseLink::builder()
            .project(1337)
            .tag_name("1.2.3 01")
            .name("test")
            .url("test.com/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_direct_asset_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases/1.2.3%2001/assets/links")
            .body_str(concat!(
                "name=test",
                "&url=test.com%2Ffile",
                "&direct_asset_path=path%2Fto%2Ffile",
            ))
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateReleaseLink::builder()
            .project(1337)
            .tag_name("1.2.3 01")
            .name("test")
            .url("test.com/file")
            .direct_asset_path("path/to/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_link_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases/1.2.3%2001/assets/links")
            .body_str(concat!(
                "name=test",
                "&url=test.com%2Ffile",
                "&link_type=other",
            ))
            .content_type("application/x-www-form-urlencoded")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateReleaseLink::builder()
            .project(1337)
            .tag_name("1.2.3 01")
            .name("test")
            .url("test.com/file")
            .link_type(LinkType::Other)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
