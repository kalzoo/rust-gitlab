// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::common::{NameOrId, SortOrder};
use crate::api::packages::{PackageStatus, PackageType};
use crate::api::{endpoint_prelude::*, ParamValue};
use derive_builder::Builder;

/// Sort orderings for packages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageOrderBy {
    /// Order by the creation date of the package.
    CreatedAt,
    /// Order by the name of the package.
    Name,
    /// Order by the version of the package.
    Version,
    /// Order by the type of the package.
    Type,
    /// Order by the project path of the package.
    ProjectPath,
}

impl PackageOrderBy {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            PackageOrderBy::CreatedAt => "created_at",
            PackageOrderBy::Name => "name",
            PackageOrderBy::Version => "version",
            PackageOrderBy::Type => "type",
            PackageOrderBy::ProjectPath => "project_path",
        }
    }
}

impl ParamValue<'static> for PackageOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// List packages within a group
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Packages<'a> {
    /// The project to query for the packages.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Exclude packages from projects of subgroups
    #[builder(default)]
    exclude_subgroups: Option<bool>,

    /// How to order returned results.
    #[builder(default)]
    order_by: Option<PackageOrderBy>,

    /// The sort order of returned packages.
    #[builder(default)]
    sort: Option<SortOrder>,

    /// Filter results by package type.
    #[builder(default)]
    package_type: Option<PackageType>,

    /// Filter packages by a fuzzy search on the name.
    #[builder(setter(into), default)]
    package_name: Option<Cow<'a, str>>,

    /// Include versionless packages.
    #[builder(default)]
    include_versionless: Option<bool>,

    /// Filter based on the status of the package.
    #[builder(default)]
    status: Option<PackageStatus>,
}

impl<'a> Packages<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PackagesBuilder<'a> {
        PackagesBuilder::default()
    }
}

impl<'a> Endpoint for Packages<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/packages", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("exclude_subgroups", self.exclude_subgroups)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("package_type", self.package_type)
            .push_opt("package_name", self.package_name.as_ref())
            .push_opt("include_versionless", self.include_versionless)
            .push_opt("status", self.status);

        params
    }
}

impl<'a> Pageable for Packages<'a> {}

#[cfg(test)]
mod tests {
    use super::{PackageOrderBy, PackageStatus, PackageType};
    use crate::{
        api::{
            self,
            common::SortOrder,
            groups::packages::packages::{Packages, PackagesBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn order_by_as_str() {
        let items = &[
            (PackageOrderBy::CreatedAt, "created_at"),
            (PackageOrderBy::Name, "name"),
            (PackageOrderBy::Type, "type"),
            (PackageOrderBy::Version, "version"),
            (PackageOrderBy::ProjectPath, "project_path"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = Packages::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, PackagesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Packages::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder().project(1337).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_exclude_subgroups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("exclude_subgroups", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .exclude_subgroups(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("order_by", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .order_by(PackageOrderBy::Name)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_package_type() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("package_type", "conan")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .package_type(PackageType::Conan)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_package_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("package_name", "test")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .package_name("test")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_versionless() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("include_versionless", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .include_versionless(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("groups/1337/packages")
            .add_query_params(&[("status", "processing")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Packages::builder()
            .project(1337)
            .status(PackageStatus::Processing)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
