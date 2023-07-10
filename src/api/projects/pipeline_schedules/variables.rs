// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project pipeline schedule variable API endpoints.
//!
//! These endpoints are used for querying CI pipeline schedule variables.

mod create;
mod delete;
mod edit;

pub use self::create::CreatePipelineScheduleVariable;
pub use self::create::CreatePipelineScheduleVariableBuilder;
pub use self::create::CreatePipelineScheduleVariableBuilderError;

pub use self::delete::DeletePipelineScheduleVariable;
pub use self::delete::DeletePipelineScheduleVariableBuilder;
pub use self::delete::DeletePipelineScheduleVariableBuilderError;

pub use self::edit::EditPipelineScheduleVariable;
pub use self::edit::EditPipelineScheduleVariableBuilder;
pub use self::edit::EditPipelineScheduleVariableBuilderError;
