// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// ht0w.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE1tp://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use derive_builder::Builder;

/// Deletes a project package.
#[derive(Debug, Builder, Clone)]
pub struct DeletePackage<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// ID of a package.
    package_id: u64,
}

impl<'a> DeletePackage<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeletePackageBuilder<'a> {
        DeletePackageBuilder::default()
    }
}

impl<'a> Endpoint for DeletePackage<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/packages/{}", self.project, self.package_id).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{
            self,
            projects::packages::delete::{DeletePackage, DeletePackageBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_needed() {
        let err = DeletePackage::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeletePackageBuilderError, "project");
    }

    #[test]
    fn package_id_is_needed() {
        let err = DeletePackage::builder().project(1337).build().unwrap_err();
        crate::test::assert_missing_field!(err, DeletePackageBuilderError, "package_id");
    }

    #[test]
    fn project_and_package_id_is_sufficient() {
        DeletePackage::builder()
            .project(1)
            .package_id(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/1337/packages/1")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeletePackage::builder()
            .project(1337)
            .package_id(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
