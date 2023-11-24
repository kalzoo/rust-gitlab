// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Upload a package file of a single package.
#[derive(Debug, Builder, Clone)]
pub struct GetPackageFile<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The package name.
    ///
    /// It can contain only lowercase letters (a-z),
    /// uppercase letter (A-Z), numbers (0-9), dots (.),
    /// hyphens (-), or underscores (_).
    #[builder(setter(into))]
    package_name: Cow<'a, str>,

    /// The package version.
    ///
    /// The following regex validates this: \A(\.?[\w\+-]+\.?)+\z.
    #[builder(setter(into))]
    package_version: Cow<'a, str>,

    /// The filename.
    ///
    /// It can contain only lowercase letters (a-z),
    /// uppercase letter (A-Z), numbers (0-9), dots (.),
    /// hyphens (-), or underscores (_).
    #[builder(setter(into))]
    file_name: Cow<'a, str>,
}

impl<'a> GetPackageFile<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetPackageFileBuilder<'a> {
        GetPackageFileBuilder::default()
    }
}

impl<'a> Endpoint for GetPackageFile<'a> {
    fn method(&self) -> Method {
        Method::GET
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
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{
            self,
            projects::packages::generic::get::{GetPackageFile, GetPackageFileBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_needed() {
        let err = GetPackageFile::builder()
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GetPackageFileBuilderError, "project");
    }

    #[test]
    fn package_name_is_needed() {
        let err = GetPackageFile::builder()
            .project(1337)
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GetPackageFileBuilderError, "package_name");
    }

    #[test]
    fn package_version_is_needed() {
        let err = GetPackageFile::builder()
            .project(1337)
            .package_name("test_package")
            .file_name("test_file.zip")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GetPackageFileBuilderError, "package_version");
    }

    #[test]
    fn file_name_is_needed() {
        let err = GetPackageFile::builder()
            .project(1337)
            .package_name("test_package")
            .package_version("1.2.3")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, GetPackageFileBuilderError, "file_name");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        GetPackageFile::builder()
            .project(1)
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/1337/packages/generic/test_package/1.2.3/test_file.zip")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPackageFile::builder()
            .project(1337)
            .package_name("test_package")
            .package_version("1.2.3")
            .file_name("test_file.zip")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_pathed() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/1337/packages/generic/test_package%2Fwith_dir/subpkg%2F1.2.3/test_dir%2Ffile.zip")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPackageFile::builder()
            .project(1337)
            .package_name("test_package/with_dir")
            .package_version("subpkg/1.2.3")
            .file_name("test_dir/file.zip")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
