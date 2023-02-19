// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Get a list of package files of a single package.
#[derive(Debug, Builder, Clone)]
pub struct PackageFiles<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// ID of a package.
    package_id: u64,
}

impl<'a> PackageFiles<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PackageFilesBuilder<'a> {
        PackageFilesBuilder::default()
    }
}

impl<'a> Endpoint for PackageFiles<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/packages/{}/package_files",
            self.project, self.package_id,
        )
        .into()
    }
}

impl<'a> Pageable for PackageFiles<'a> {}

#[cfg(test)]
mod tests {
    use crate::{
        api::{
            self,
            projects::packages::package_files::{PackageFiles, PackageFilesBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_needed() {
        let err = PackageFiles::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PackageFilesBuilderError, "project");
    }

    #[test]
    fn package_id_is_needed() {
        let err = PackageFiles::builder().project(1337).build().unwrap_err();
        crate::test::assert_missing_field!(err, PackageFilesBuilderError, "package_id");
    }

    #[test]
    fn required_parameter_are_sufficient() {
        PackageFiles::builder()
            .project(1)
            .package_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1337/packages/1/package_files")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = PackageFiles::builder()
            .project(1337)
            .package_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
