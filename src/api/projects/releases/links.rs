// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project release links API endpoints.
//!
//! These endpoints are used for querying, creating, and deleting project release links.

mod common;
mod create;
mod delete;
mod link;
mod links;
mod update;

pub use common::LinkType;

pub use links::ListReleaseLinks;
pub use links::ListReleaseLinksBuilder;
pub use links::ListReleaseLinksBuilderError;

pub use link::GetReleaseLink;
pub use link::GetReleaseLinkBuilder;
pub use link::GetReleaseLinkBuilderError;

pub use create::CreateReleaseLink;
pub use create::CreateReleaseLinkBuilder;
pub use create::CreateReleaseLinkBuilderError;

pub use update::UpdateReleaseLink;
pub use update::UpdateReleaseLinkBuilder;
pub use update::UpdateReleaseLinkBuilderError;

pub use delete::DeleteReleaseLink;
pub use delete::DeleteReleaseLinkBuilder;
pub use delete::DeleteReleaseLinkBuilderError;
