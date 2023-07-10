// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project pipeline schedule API endpoints.
//!
//! These endpoints are used for querying CI pipeline schedules.

mod create;
mod delete;
mod edit;
mod pipelines;
mod play;
mod schedule;
mod schedules;
mod take_ownership;
pub mod variables;

pub use self::create::CreatePipelineSchedule;
pub use self::create::CreatePipelineScheduleBuilder;
pub use self::create::CreatePipelineScheduleBuilderError;
pub use self::create::PipelineScheduleCron;
pub use self::create::PipelineScheduleCronError;
pub use self::create::PipelineScheduleTimeZone;

pub use self::delete::DeletePipelineSchedule;
pub use self::delete::DeletePipelineScheduleBuilder;
pub use self::delete::DeletePipelineScheduleBuilderError;

pub use self::edit::EditPipelineSchedule;
pub use self::edit::EditPipelineScheduleBuilder;
pub use self::edit::EditPipelineScheduleBuilderError;

pub use self::pipelines::PipelineSchedulePipelines;
pub use self::pipelines::PipelineSchedulePipelinesBuilder;
pub use self::pipelines::PipelineSchedulePipelinesBuilderError;

pub use self::play::PlayPipelineSchedule;
pub use self::play::PlayPipelineScheduleBuilder;
pub use self::play::PlayPipelineScheduleBuilderError;

pub use self::schedule::PipelineSchedule;
pub use self::schedule::PipelineScheduleBuilder;
pub use self::schedule::PipelineScheduleBuilderError;

pub use self::schedules::PipelineScheduleScope;
pub use self::schedules::PipelineSchedules;
pub use self::schedules::PipelineSchedulesBuilder;
pub use self::schedules::PipelineSchedulesBuilderError;

pub use self::take_ownership::TakePipelineScheduleOwnership;
pub use self::take_ownership::TakePipelineScheduleOwnershipBuilder;
pub use self::take_ownership::TakePipelineScheduleOwnershipBuilderError;
