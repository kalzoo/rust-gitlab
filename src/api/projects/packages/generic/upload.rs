// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;
use derive_builder::Builder;

/// The package status.
///
/// It can be default (default) or hidden. Hidden packages do not appear in the UI or package API
/// list endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UploadPackageStatus {
    /// All packages that appear in the UI
    Default,
    /// All packages that don't appear in the UI
    Hidden,
}

#[allow(clippy::derivable_impls)]
impl Default for UploadPackageStatus {
    fn default() -> Self {
        // XXX(rust-1.62): use `#[default]`
        UploadPackageStatus::Default
    }
}

impl UploadPackageStatus {
    /// The status as a query parameter
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Hidden => "hidden",
        }
    }
}

impl ParamValue<'static> for UploadPackageStatus {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// The response payload.
///
/// By default, the response is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UploadPackageSelect {
    /// Returns details of the package file record created by this request
    PackageFile,
}

impl UploadPackageSelect {
    fn as_str(self) -> &'static str {
        match self {
            Self::PackageFile => "package_file",
        }
    }
}

impl ParamValue<'static> for UploadPackageSelect {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Upload a package file of a single package.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct UploadPackageFile<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The package name.
    ///
    /// It can contain only lowercase letters (a-z), uppercase letter (A-Z), numbers (0-9), dots
    /// (.), hyphens (-), or underscores (_).
    #[builder(setter(into))]
    package_name: Cow<'a, str>,

    /// The package version.
    ///
    /// The following regex validates this: `\A(\.?[\w\+-]+\.?)+\z`.
    #[builder(setter(into))]
    package_version: Cow<'a, str>,

    /// The filename.
    ///
    /// It can contain only lowercase letters (a-z), uppercase letter (A-Z), numbers (0-9), dots
    /// (.), hyphens (-), or underscores (_).
    #[builder(setter(into))]
    file_name: Cow<'a, str>,

    /// The package status
    ///
    /// It can be default (default) or hidden. Hidden packages do not appear in the UI or package
    /// API list endpoints.
    #[builder(default)]
    status: Option<UploadPackageStatus>,

    /// The response payload.
    ///
    /// By default, the response is empty. Valid values are: package_file. package_file returns
    /// details of the package file record created by this request.
    #[builder(default)]
    select: Option<UploadPackageSelect>,

    /// The file as an array of bytes.
    #[builder(setter(into))]
    contents: Cow<'a, [u8]>,
}

impl<'a> UploadPackageFile<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UploadPackageFileBuilder<'a> {
        UploadPackageFileBuilder::default()
    }
}

impl<'a> Endpoint for UploadPackageFile<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/packages/generic/{}/{}/{}",
            self.project,
            common::path_escaped(self.package_name.as_ref()),
            common::path_escaped(self.package_version.as_ref()),
            common::path_escaped(self.file_name.as_ref()),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("status", self.status)
            .push_opt("select", self.select);

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(Some(("application/octet-stream", self.contents.to_vec())))
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{
            self,
            projects::packages::generic::upload::{
                UploadPackageFile, UploadPackageFileBuilderError,
            },
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    use super::{UploadPackageSelect, UploadPackageStatus};

    #[test]
    fn project_is_needed() {
        let err = UploadPackageFile::builder()
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .contents(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UploadPackageFileBuilderError, "project");
    }

    #[test]
    fn package_name_is_needed() {
        let err = UploadPackageFile::builder()
            .project(1)
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .contents(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UploadPackageFileBuilderError, "package_name");
    }

    #[test]
    fn package_version_is_needed() {
        let err = UploadPackageFile::builder()
            .project(1)
            .package_name("test_package")
            .file_name("test_file.zip")
            .contents(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UploadPackageFileBuilderError, "package_version");
    }

    #[test]
    fn file_name_is_needed() {
        let err = UploadPackageFile::builder()
            .project(1)
            .package_name("test_package")
            .package_version("1.2.3")
            .contents(&b"contents"[..])
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UploadPackageFileBuilderError, "file_name");
    }

    #[test]
    fn contents_is_needed() {
        let err = UploadPackageFile::builder()
            .project(1)
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, UploadPackageFileBuilderError, "contents");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        UploadPackageFile::builder()
            .project(1)
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .contents(&b"contents"[..])
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let contents = &b"contents"[..];
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/1337/packages/generic/test%20package/1.2.3%201/test%20file.zip")
            .body(contents.to_vec())
            .content_type("application/octet-stream")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UploadPackageFile::builder()
            .project(1337)
            .package_name("test package")
            .package_version("1.2.3 1")
            .file_name("test file.zip")
            .contents(contents)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let contents = &b"contents"[..];
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/1337/packages/generic/test%20package/1.2.3%201/test%20file.zip")
            .add_query_params(&[("status", "hidden")])
            .body(contents.to_vec())
            .content_type("application/octet-stream")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UploadPackageFile::builder()
            .project(1337)
            .package_name("test package")
            .package_version("1.2.3 1")
            .file_name("test file.zip")
            .contents(contents)
            .status(UploadPackageStatus::Hidden)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_select() {
        let contents = &b"contents"[..];
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("projects/1337/packages/generic/test%20package/1.2.3%201/test%20file.zip")
            .add_query_params(&[("select", "package_file")])
            .body(contents.to_vec())
            .content_type("application/octet-stream")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UploadPackageFile::builder()
            .project(1337)
            .package_name("test package")
            .package_version("1.2.3 1")
            .file_name("test file.zip")
            .contents(contents)
            .select(UploadPackageSelect::PackageFile)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
