// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project release API endpoints.
//!
//! These endpoints are used for querying project releases.

mod create;
pub mod links;
mod releases;

pub use self::releases::ProjectReleaseOrderBy;
pub use self::releases::ProjectReleases;
pub use self::releases::ProjectReleasesBuilder;
pub use self::releases::ProjectReleasesBuilderError;

pub use self::create::CreateRelease;
pub use self::create::CreateReleaseAssetLinks;
pub use self::create::CreateReleaseAssetLinksBuilder;
pub use self::create::CreateReleaseAssetLinksBuilderError;
pub use self::create::CreateReleaseBuilder;
pub use self::create::CreateReleaseBuilderError;
