// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project Packages API endpoints.
//!
//! These endpoints are used for querying packages.

mod delete;
pub mod generic;
mod package;
pub mod package_files;
mod packages;

pub use self::delete::DeletePackage;
pub use self::delete::DeletePackageBuilder;
pub use self::delete::DeletePackageBuilderError;

pub use self::package::Package;
pub use self::package::PackageBuilder;
pub use self::package::PackageBuilderError;

pub use self::packages::PackageOrderBy;
pub use self::packages::Packages;
pub use self::packages::PackagesBuilder;
pub use self::packages::PackagesBuilderError;
