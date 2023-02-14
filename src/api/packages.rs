// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Packages API types.
//!
//! These types are used for querying packages from projects or groups.

use std::borrow::Cow;

use super::ParamValue;

/// Package Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageStatus {
    /// The default filter.
    Default,
    /// Return hidden packaegs.
    Hidden,
    /// Return in-progress packages.
    Processing,
    /// Return packages with errors.
    Error,
    /// Return packages which are pending destruction.
    PendingDestruction,
}

impl PackageStatus {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            PackageStatus::Default => "default",
            PackageStatus::Hidden => "hidden",
            PackageStatus::Processing => "processing",
            PackageStatus::Error => "error",
            PackageStatus::PendingDestruction => "pending_destruction",
        }
    }
}

impl ParamValue<'static> for PackageStatus {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Package types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageType {
    /// Return `conan` (C and C++) packages.
    Conan,
    /// Return `maven` (Java) packages.
    Maven,
    /// Return `npm` (NodeJS) packages.
    Npm,
    /// Return `pypi` (Python) packages.
    Pypi,
    /// Return `composer` (PHP) packages.
    Composer,
    /// Return `nuget` (C#) packages.
    Nuget,
    /// Return `helm` (Kubernetes) packages.
    Helm,
    /// Return `terraform` (Terraform) packages.
    TerraformModule,
    /// Return `golang` (Go) packages.
    GoLang,
}

impl PackageType {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            PackageType::Conan => "conan",
            PackageType::Maven => "maven",
            PackageType::Npm => "npm",
            PackageType::Pypi => "pypi",
            PackageType::Composer => "composer",
            PackageType::Nuget => "nuget",
            PackageType::Helm => "helm",
            PackageType::TerraformModule => "terraform_module",
            PackageType::GoLang => "golang",
        }
    }
}

impl ParamValue<'static> for PackageType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use super::{PackageStatus, PackageType};

    #[test]
    fn package_type_as_str() {
        let items = &[
            (PackageType::Conan, "conan"),
            (PackageType::Maven, "maven"),
            (PackageType::Npm, "npm"),
            (PackageType::Pypi, "pypi"),
            (PackageType::Composer, "composer"),
            (PackageType::Nuget, "nuget"),
            (PackageType::Helm, "helm"),
            (PackageType::TerraformModule, "terraform_module"),
            (PackageType::GoLang, "golang"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn package_status_as_str() {
        let items = &[
            (PackageStatus::Default, "default"),
            (PackageStatus::Hidden, "hidden"),
            (PackageStatus::Processing, "processing"),
            (PackageStatus::Error, "error"),
            (PackageStatus::PendingDestruction, "pending_destruction"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }
}
