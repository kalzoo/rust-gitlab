// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request note award API endpoints.
//!
//! These endpoints are used for querying project merge request note awards.

mod award;
mod awards;
mod create;
mod delete;

pub use self::award::MergeRequestNoteAward;
pub use self::award::MergeRequestNoteAwardBuilder;
pub use self::award::MergeRequestNoteAwardBuilderError;

pub use self::awards::MergeRequestNoteAwards;
pub use self::awards::MergeRequestNoteAwardsBuilder;
pub use self::awards::MergeRequestNoteAwardsBuilderError;

pub use self::create::CreateMergeRequestNoteAward;
pub use self::create::CreateMergeRequestNoteAwardBuilder;
pub use self::create::CreateMergeRequestNoteAwardBuilderError;

pub use self::delete::DeleteMergeRequestNoteAward;
pub use self::delete::DeleteMergeRequestNoteAwardBuilder;
pub use self::delete::DeleteMergeRequestNoteAwardBuilderError;
