// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;
use crate::api::users::ExternalProvider;

/// Password settings for a new user.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NewUserPassword<'a> {
    /// A literal password.
    Password(Cow<'a, str>),
    /// Send a password reset email to the user.
    Reset,
    /// Set a random initial password.
    Random,
    /// Set a random initial password and send a reset email.
    ResetRandom,
}

impl<'a, T> From<T> for NewUserPassword<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(password: T) -> Self {
        Self::Password(password.into())
    }
}

impl<'a> NewUserPassword<'a> {
    fn add_query<'b>(&'b self, params: &mut FormParams<'b>) {
        match self {
            Self::Password(password) => params.push("password", password),
            Self::Reset => params.push("reset_password", true),
            Self::Random => params.push("force_random_password", true),
            Self::ResetRandom => {
                params
                    .push("reset_password", true)
                    .push("force_random_password", true)
            },
        };
    }
}

/// Create a new user on an instance.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CreateUser<'a> {
    /// The email of the user.
    #[builder(setter(into))]
    email: Cow<'a, str>,
    /// The name of the user.
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// The username of the user.
    #[builder(setter(into))]
    username: Cow<'a, str>,
    /// The password of the user.
    #[builder(setter(into))]
    password: NewUserPassword<'a>,

    /// Skip confirmation for newly created user
    #[builder(default)]
    skip_confirmation: Option<bool>,

    /// Whether the user is an administrator or not.
    #[builder(default)]
    admin: Option<bool>,
    /// Whether the user is an auditor or not.
    #[builder(default)]
    auditor: Option<bool>,

    /// Whether the user is provided by an external entity or not.
    #[builder(default)]
    external: Option<bool>,
    /// The ID of the group for SAML.
    #[builder(default)]
    group_id_for_saml: Option<u64>,
    /// Administrator notes for the user.
    #[builder(setter(into), default)]
    note: Option<Cow<'a, str>>,

    /// Set the external provider identity for the user.
    #[builder(default)]
    external_provider: Option<ExternalProvider<'a>>,

    /// The ID of the color scheme to use for the user.
    #[builder(default)]
    color_scheme_id: Option<u64>,
    /// Whether the user's profile is set to private or not.
    #[builder(default)]
    private_profile: Option<bool>,
    /// The ID of the theme to use for the user.
    #[builder(default)]
    theme_id: Option<u64>,
    /// Whether the user prefers viewing diffs file-by-file or not.
    #[builder(default)]
    view_diffs_file_by_file: Option<bool>,

    /// Whether the user can create groups or not.
    #[builder(default)]
    can_create_group: Option<bool>,
    /// Extra minutes on shared runners for the user.
    #[builder(default)]
    extra_shared_runners_minutes_limit: Option<u64>,
    /// The number of projects the user may create.
    #[builder(default)]
    projects_limit: Option<u64>,
    /// The limit shared runners usage (in minutes) for the user.
    #[builder(default)]
    shared_runners_minutes_limit: Option<u64>,

    // TODO: Figure out how to actually use this.
    // avatar: ???,
    /// Biographical information about the user.
    #[builder(setter(into), default)]
    bio: Option<Cow<'a, str>>,
    /// The user's commit email address.
    #[builder(setter(into), default)]
    commit_email: Option<Cow<'a, str>>,
    /// The user's LinkedIn URL.
    #[builder(setter(into), default)]
    linkedin: Option<Cow<'a, str>>,
    /// The user's location.
    #[builder(setter(into), default)]
    location: Option<Cow<'a, str>>,
    /// The user's organization.
    #[builder(setter(into), default)]
    organization: Option<Cow<'a, str>>,
    /// The user's pronouns.
    #[builder(setter(into), default)]
    pronouns: Option<Cow<'a, str>>,
    /// The user's public email address.
    #[builder(setter(into), default)]
    public_email: Option<Cow<'a, str>>,
    /// The user's Skype ID.
    #[builder(setter(into), default)]
    skype: Option<Cow<'a, str>>,
    /// The user's Twitter ID.
    #[builder(setter(into), default)]
    twitter: Option<Cow<'a, str>>,
    /// The user's Discord ID.
    #[builder(setter(into), default)]
    discord: Option<Cow<'a, str>>,
    /// The user's website URL.
    #[builder(setter(into), default)]
    website_url: Option<Cow<'a, str>>,
}

impl<'a> CreateUser<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateUserBuilder<'a> {
        CreateUserBuilder::default()
    }
}

impl<'a> Endpoint for CreateUser<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("email", self.email.as_ref())
            .push("name", self.name.as_ref())
            .push("username", self.username.as_ref())
            .push_opt("skip_confirmation", self.skip_confirmation)
            .push_opt("admin", self.admin)
            .push_opt("auditor", self.auditor)
            .push_opt("external", self.external)
            .push_opt("group_id_for_saml", self.group_id_for_saml)
            .push_opt("note", self.note.as_ref())
            .push_opt("color_scheme_id", self.color_scheme_id)
            .push_opt("private_profile", self.private_profile)
            .push_opt("theme_id", self.theme_id)
            .push_opt("view_diffs_file_by_file", self.view_diffs_file_by_file)
            .push_opt("can_create_group", self.can_create_group)
            .push_opt(
                "extra_shared_runners_minutes_limit",
                self.extra_shared_runners_minutes_limit,
            )
            .push_opt("projects_limit", self.projects_limit)
            .push_opt(
                "shared_runners_minutes_limit",
                self.shared_runners_minutes_limit,
            )
            .push_opt("bio", self.bio.as_ref())
            .push_opt("commit_email", self.commit_email.as_ref())
            .push_opt("linkedin", self.linkedin.as_ref())
            .push_opt("location", self.location.as_ref())
            .push_opt("organization", self.organization.as_ref())
            .push_opt("pronouns", self.pronouns.as_ref())
            .push_opt("public_email", self.public_email.as_ref())
            .push_opt("skype", self.skype.as_ref())
            .push_opt("twitter", self.twitter.as_ref())
            .push_opt("discord", self.discord.as_ref())
            .push_opt("website_url", self.website_url.as_ref());

        self.password.add_query(&mut params);

        if let Some(value) = self.external_provider.as_ref() {
            params
                .push("extern_uid", &value.uid)
                .push("provider", &value.name);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::users::{
        CreateUser, CreateUserBuilderError, ExternalProvider, NewUserPassword,
    };
    use http::Method;

    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn email_is_necessary() {
        let err = CreateUser::builder()
            .name("name")
            .username("username")
            .password(NewUserPassword::Reset)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateUserBuilderError, "email");
    }

    #[test]
    fn name_is_necessary() {
        let err = CreateUser::builder()
            .email("email@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateUserBuilderError, "name");
    }

    #[test]
    fn username_is_necessary() {
        let err = CreateUser::builder()
            .name("name")
            .email("email@example.com")
            .password(NewUserPassword::Reset)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateUserBuilderError, "username");
    }

    #[test]
    fn password_is_necessary() {
        let err = CreateUser::builder()
            .email("email@example.com")
            .name("name")
            .username("username")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, CreateUserBuilderError, "password");
    }

    #[test]
    fn sufficient_parameters() {
        CreateUser::builder()
            .email("email@example.com")
            .name("name")
            .username("username")
            .password(NewUserPassword::Reset)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&password=test-password",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password("test-password")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_password_reset() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_password_random() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&force_random_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Random)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_password_reset_random() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&reset_password=true",
                "&force_random_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::ResetRandom)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skip_confirmation() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&skip_confirmation=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .skip_confirmation(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_admin() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&admin=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .admin(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_auditor() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&auditor=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .auditor(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&external=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .external(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_group_id_for_saml() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&group_id_for_saml=1",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .group_id_for_saml(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_note() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&note=admin+notes",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .note("admin notes")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_external_provider() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&reset_password=true",
                "&extern_uid=foobar",
                "&provider=magic",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .external_provider(
                ExternalProvider::builder()
                    .uid("foobar")
                    .name("magic")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_color_scheme_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&color_scheme_id=1",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .color_scheme_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_private_profile() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&private_profile=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .private_profile(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_theme_id() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&theme_id=1",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .theme_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_view_diffs_file_by_file() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&view_diffs_file_by_file=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .view_diffs_file_by_file(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_can_create_group() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&can_create_group=true",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .can_create_group(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_extra_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&extra_shared_runners_minutes_limit=10",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .extra_shared_runners_minutes_limit(10)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_projects_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&projects_limit=100",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .projects_limit(100)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_shared_runners_minutes_limit() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&shared_runners_minutes_limit=1000",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .shared_runners_minutes_limit(1000)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_bio() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&bio=bio",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .bio("bio")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_commit_email() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&commit_email=commit%40example.com",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .commit_email("commit@example.com")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_linkedin() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&linkedin=linkedin_url",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .linkedin("linkedin_url")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_location() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&location=home",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .location("home")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_organization() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&organization=mywork",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .organization("mywork")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pronouns() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&pronouns=robot",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .pronouns("robot")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_public_email() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&public_email=public%40example.com",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .public_email("public@example.com")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_skype() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&skype=skype_url",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .skype("skype_url")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_twitter() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&twitter=twitter_handle",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .twitter("twitter_handle")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_discord() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&discord=discord_username",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .discord("discord_username")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_website_url() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("users")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "email=user%40example.com",
                "&name=name",
                "&username=username",
                "&website_url=homepage",
                "&reset_password=true",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateUser::builder()
            .name("name")
            .email("user@example.com")
            .username("username")
            .password(NewUserPassword::Reset)
            .website_url("homepage")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
