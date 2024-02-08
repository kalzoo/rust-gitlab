// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::CommaSeparatedList;
use crate::api::endpoint_prelude::*;
use crate::api::runners::RunnerAccessLevel;

/// Runner metadata fields
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RunnerMetadata<'a> {
    /// The name of the runner.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The version of the runner.
    #[builder(setter(into), default)]
    version: Option<Cow<'a, str>>,
    /// The platform of the runner.
    #[builder(setter(into), default)]
    platform: Option<Cow<'a, str>>,
    /// The architecture of the runner.
    #[builder(setter(into), default)]
    architecture: Option<Cow<'a, str>>,
}

impl<'a> RunnerMetadata<'a> {
    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push_opt("info[name]", self.name.as_ref())
            .push_opt("info[version]", self.version.as_ref())
            .push_opt("info[platform]", self.platform.as_ref())
            .push_opt("info[architecture]", self.architecture.as_ref());
    }
}

/// Create a runner.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct CreateRunner<'a> {
    /// The registration token.
    #[builder(setter(into))]
    token: Cow<'a, str>,

    /// The description of the runner.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// Metadata about the runner.
    #[builder(default)]
    info: Option<RunnerMetadata<'a>>,
    /// Whether the runner should ignore new jobs or not.
    #[builder(default)]
    paused: Option<bool>,
    /// Whether the runner is locked or not.
    #[builder(default)]
    locked: Option<bool>,
    /// Whether the runner can execute untagged jobs or not.
    #[builder(default)]
    run_untagged: Option<bool>,
    /// Set the tags for the runner.
    #[builder(setter(name = "_tag_list"), default, private)]
    tag_list: Option<CommaSeparatedList<Cow<'a, str>>>,
    /// The access level of the runner.
    #[builder(default)]
    access_level: Option<RunnerAccessLevel>,
    /// The maximum timeout allowed on the runner (in seconds).
    #[builder(default)]
    maximum_timeout: Option<u64>,
    /// Maintenance note for the runner.
    ///
    /// Maximum size is 1024.
    #[builder(setter(into), default)]
    maintenance_note: Option<Cow<'a, str>>,
}

impl<'a> CreateRunner<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateRunnerBuilder<'a> {
        CreateRunnerBuilder::default()
    }
}

const MAX_MAINTENANCE_NOTE_LENGTH: usize = 1024;

impl<'a> CreateRunnerBuilder<'a> {
    /// Add a tag to the runner.
    pub fn tag<T>(&mut self, tag: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .push(tag.into());
        self
    }

    /// Add multiple tags to the runner.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tag_list
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(|t| t.into()));
        self
    }

    fn validate(&self) -> Result<(), CreateRunnerBuilderError> {
        if let Some(Some(maintenance_note)) = self.maintenance_note.as_ref() {
            if maintenance_note.len() > MAX_MAINTENANCE_NOTE_LENGTH {
                return Err(format!(
                    "`maintenance_note` may be at most {} bytes",
                    MAX_MAINTENANCE_NOTE_LENGTH,
                )
                .into());
            }
        }

        Ok(())
    }
}

impl<'a> Endpoint for CreateRunner<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "runners".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("token", &self.token)
            .push_opt("description", self.description.as_ref())
            .push_opt("paused", self.paused)
            .push_opt("locked", self.locked)
            .push_opt("run_untagged", self.run_untagged)
            .push_opt("tag_list", self.tag_list.as_ref())
            .push_opt("access_level", self.access_level)
            .push_opt("maximum_timeout", self.maximum_timeout)
            .push_opt("maintenance_note", self.maintenance_note.as_ref());

        if let Some(info) = self.info.as_ref() {
            info.add_params(&mut params);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::runners::{CreateRunner, CreateRunnerBuilderError, RunnerAccessLevel};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn token_is_required() {
        let err = CreateRunner::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CreateRunnerBuilderError, "token");
    }

    #[test]
    fn token_is_sufficient() {
        CreateRunner::builder().token("tok").build().unwrap();
    }

    #[test]
    fn maintenance_note_length() {
        use super::MAX_MAINTENANCE_NOTE_LENGTH;

        let too_long = format!("{:width$}", "note", width = MAX_MAINTENANCE_NOTE_LENGTH + 1);
        let err = CreateRunner::builder()
            .token("tok")
            .maintenance_note(too_long)
            .build()
            .unwrap_err();
        if let CreateRunnerBuilderError::ValidationError(message) = err {
            assert_eq!(
                message,
                format!(
                    "`maintenance_note` may be at most {} bytes",
                    MAX_MAINTENANCE_NOTE_LENGTH,
                )
            );
        } else {
            panic!("unexpected error: {:?}", err);
        }
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str("token=tok")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder().token("tok").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_description() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&description=desc"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .description("desc")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_info() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&paused=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .paused(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_paused() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&paused=true"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .paused(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_locked() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&locked=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .locked(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_run_untagged() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&run_untagged=false"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .run_untagged(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_tag_list() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&tag_list=tag2%2Ctag1%2Ctag3"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .tag("tag2")
            .tags(["tag1", "tag3"].iter().cloned())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_access_level() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&access_level=ref_protected"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .access_level(RunnerAccessLevel::RefProtected)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_maximum_timeout() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&maximum_timeout=3600"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .maximum_timeout(3600)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_maintenance_note() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("runners")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("token=tok", "&maintenance_note=note"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateRunner::builder()
            .token("tok")
            .maintenance_note("note")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
