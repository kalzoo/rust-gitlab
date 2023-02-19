// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Delete a package file of a single package.
#[derive(Debug, Builder, Clone)]
pub struct DeletePackageFile<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// ID of a package.
    package_id: u64,

    /// ID of a package file.
    package_file_id: u64,
}

impl<'a> DeletePackageFile<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeletePackageFileBuilder<'a> {
        DeletePackageFileBuilder::default()
    }
}

impl<'a> Endpoint for DeletePackageFile<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/packages/{}/package_files/{}",
            self.project, self.package_id, self.package_file_id,
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
            projects::packages::package_files::delete::{
                DeletePackageFile, DeletePackageFileBuilderError,
            },
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_needed() {
        let err = DeletePackageFile::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeletePackageFileBuilderError, "project");
    }

    #[test]
    fn package_id_is_needed() {
        let err = DeletePackageFile::builder()
            .project(1337)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeletePackageFileBuilderError, "package_id");
    }

    #[test]
    fn package_file_id_is_needed() {
        let err = DeletePackageFile::builder()
            .project(1337)
            .package_id(1)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, DeletePackageFileBuilderError, "package_file_id");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        DeletePackageFile::builder()
            .project(1)
            .package_id(1)
            .package_file_id(2)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/1337/packages/1/package_files/2")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeletePackageFile::builder()
            .project(1337)
            .package_id(1)
            .package_file_id(2)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
