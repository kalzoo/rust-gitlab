// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde_json::json;

use crate::api::projects::releases::links::LinkType;

/// Asset link.
///
/// Used to create permalinks for your release.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateReleaseAssetLinks<'a> {
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
    #[builder(default)]
    link_type: Option<LinkType>,
}

impl<'a> CreateReleaseAssetLinks<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateReleaseAssetLinksBuilder<'a> {
        CreateReleaseAssetLinksBuilder::default()
    }

    fn as_json(&self) -> serde_json::Value {
        JsonParams::clean(json!({
            "name": self.name,
            "url": self.url,
            "direct_asset_path": self.direct_asset_path,
            "link_type": self.link_type.map(|lt| lt.as_str()),
        }))
    }
}

/// Creates a release.
///
/// Developer level access to the project is required to create a release.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateRelease<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The name of the link.
    ///
    /// Link names must be unique in the release.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,

    /// The tag associated with the release.
    #[builder(setter(into))]
    tag_name: Cow<'a, str>,

    /// Message to use if creating a new annotated tag.
    #[builder(setter(into), default)]
    tag_message: Option<Cow<'a, str>>,

    /// The description of the release.
    ///
    /// You can use Markdown.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,

    /// If a tag specified in tag_name doesn’t exist,
    /// the release is created from ref and tagged with tag_name.
    ///
    /// It can be a commit SHA, another tag name, or a branch name.
    #[builder(setter(into), default)]
    ref_sha: Option<Cow<'a, str>>,

    /// The title of each milestone the release is associated with.
    #[builder(setter(name = "_milestones"), default, private)]
    milestones: Vec<Cow<'a, str>>,

    /// An array of assets links.
    #[builder(setter(name = "_assets"), default, private)]
    assets: Vec<CreateReleaseAssetLinks<'a>>,

    /// Date and time for the release.
    ///
    /// Defaults to the current time.
    /// Only provide this field if creating an upcoming
    /// or historical release.
    #[builder(default)]
    released_at: Option<DateTime<Utc>>,
}

impl<'a> CreateRelease<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateReleaseBuilder<'a> {
        CreateReleaseBuilder::default()
    }

    /// Creates a JSON string of the data for the endpoint
    fn as_json(&self) -> serde_json::Value {
        JsonParams::clean(json!({
            "name": self.name,
            "tag_name": self.tag_name,
            "tag_message": self.tag_message,
            "description": self.description,
            "ref": self.ref_sha,
            "milestones": self.milestones,
            "released_at": self.released_at,
            "assets": JsonParams::clean(json!({
                "links": self.assets
                    .iter()
                    .map(|a| a.as_json())
                    .collect::<Vec<_>>()
            })),
        }))
    }
}

impl<'a> CreateReleaseBuilder<'a> {
    /// The title of a milestone the release is associated with.
    pub fn milestone<M>(&mut self, milestone: M) -> &mut Self
    where
        M: Into<Cow<'a, str>>,
    {
        self.milestones
            .get_or_insert_with(Vec::new)
            .push(milestone.into());
        self
    }

    /// The title of milestones the release is associated with.
    pub fn milestones<I, M>(&mut self, milestones: I) -> &mut Self
    where
        I: Iterator<Item = M>,
        M: Into<Cow<'a, str>>,
    {
        self.milestones
            .get_or_insert_with(Vec::new)
            .extend(milestones.map(Into::into));
        self
    }

    /// A link to an asset in the release.
    pub fn asset(&mut self, asset: CreateReleaseAssetLinks<'a>) -> &mut Self {
        self.assets.get_or_insert_with(Vec::new).push(asset);
        self
    }

    /// An iterator over links to assets in the release.
    pub fn assets<I>(&mut self, assets: I) -> &mut Self
    where
        I: Iterator<Item = CreateReleaseAssetLinks<'a>>,
    {
        self.assets.get_or_insert_with(Vec::new).extend(assets);
        self
    }
}

impl<'a> Endpoint for CreateRelease<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/releases", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        JsonParams::into_body(&self.as_json())
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use http::Method;

    use crate::{
        api::{
            self,
            projects::releases::{CreateReleaseAssetLinks, CreateReleaseAssetLinksBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::{CreateRelease, CreateReleaseBuilderError};

    #[test]
    fn assets_url_is_needed() {
        let err = CreateReleaseAssetLinks::builder()
            .name("test")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseAssetLinksBuilderError, "url");
    }

    #[test]
    fn assets_name_is_needed() {
        let err = CreateReleaseAssetLinks::builder()
            .url("https://img.com/img.png")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseAssetLinksBuilderError, "name");
    }

    #[test]
    fn assets_required_parameter_are_sufficient() {
        CreateReleaseAssetLinks::builder()
            .name("test")
            .url("https://test.com/")
            .build()
            .unwrap();
    }

    #[test]
    fn project_is_needed() {
        let err = CreateRelease::builder()
            .tag_name("1.2.3")
            .build()
            .unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseBuilderError, "project");
    }

    #[test]
    fn tag_name_is_needed() {
        let err = CreateRelease::builder().project(123).build().unwrap_err();

        crate::test::assert_missing_field!(err, CreateReleaseBuilderError, "tag_name");
    }

    #[test]
    fn project_and_tag_name_are_sufficient() {
        CreateRelease::builder()
            .project(1)
            .tag_name("1.2.3")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str("{\"tag_name\":\"1.2.3\"}")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"name\":\"Test\",",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .name("Test")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_message() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"tag_message\":\"Test message\",",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .tag_message("Test message")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"description\":\"Test description\",",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .description("Test description")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"ref\":\"abfc1234\",",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .ref_sha("abfc1234")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_milestones() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"milestones\":[",
                "\"milestone_1\",",
                "\"milestone_2\",",
                "\"milestone_3\"",
                "],",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .milestones(["milestone_1", "milestone_2"].iter().copied())
            .milestone("milestone_3")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_released_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"released_at\":\"2023-12-16T12:00:00Z\",",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .released_at(
                DateTime::parse_from_rfc3339("2023-12-16T12:00:00Z")
                    .unwrap()
                    .into(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assets() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"assets\":",
                "{",
                "\"links\":",
                "[",
                "{",
                "\"name\":\"Test url 1\",",
                "\"url\":\"https://test.com/test-1.zip\"",
                "},",
                "{",
                "\"name\":\"Test url 2\",",
                "\"url\":\"https://test.com/test-2.zip\"",
                "},",
                "{",
                "\"name\":\"Test url 3\",",
                "\"url\":\"https://test.com/test-3.zip\"",
                "}",
                "]",
                "},",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .asset(
                CreateReleaseAssetLinks::builder()
                    .name("Test url 1")
                    .url("https://test.com/test-1.zip")
                    .build()
                    .unwrap(),
            )
            .assets(
                [
                    CreateReleaseAssetLinks::builder()
                        .name("Test url 2")
                        .url("https://test.com/test-2.zip")
                        .build()
                        .unwrap(),
                    CreateReleaseAssetLinks::builder()
                        .name("Test url 3")
                        .url("https://test.com/test-3.zip")
                        .build()
                        .unwrap(),
                ]
                .into_iter(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assets_direct_asset_path() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"assets\":",
                "{",
                "\"links\":",
                "[",
                "{",
                "\"direct_asset_path\":\"bin/test.zip\",",
                "\"name\":\"Test url\",",
                "\"url\":\"https://test.com/test.zip\"",
                "}",
                "]",
                "},",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .asset(
                CreateReleaseAssetLinks::builder()
                    .name("Test url")
                    .url("https://test.com/test.zip")
                    .direct_asset_path("bin/test.zip")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_assets_link_type() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/1337/releases")
            .content_type("application/json")
            .body_str(concat!(
                "{",
                "\"assets\":",
                "{",
                "\"links\":",
                "[",
                "{",
                "\"link_type\":\"other\",",
                "\"name\":\"Test url\",",
                "\"url\":\"https://test.com/test.zip\"",
                "}",
                "]",
                "},",
                "\"tag_name\":\"1.2.3\"",
                "}",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRelease::builder()
            .project(1337)
            .tag_name("1.2.3")
            .asset(
                CreateReleaseAssetLinks::builder()
                    .name("Test url")
                    .url("https://test.com/test.zip")
                    .link_type(api::projects::releases::links::LinkType::Other)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
