// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project Packages Package Generic API endpoints.
//!
//! These endpoints are used for uploading and retrieving packages files of a generic package.

mod get;
mod upload;

pub use self::upload::UploadPackageFile;
pub use self::upload::UploadPackageFileBuilder;
pub use self::upload::UploadPackageFileBuilderError;
pub use self::upload::UploadPackageSelect;
pub use self::upload::UploadPackageStatus;

pub use self::get::GetPackageFile;
pub use self::get::GetPackageFileBuilder;
pub use self::get::GetPackageFileBuilderError;
