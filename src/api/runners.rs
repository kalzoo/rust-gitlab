// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Runner-related API endpoints
//!
//! These endpoints are used for querying and modifying CI runners and their resources.

mod all_runners;
mod edit;
mod runner;
mod runners;

pub use self::all_runners::AllRunners;
pub use self::all_runners::AllRunnersBuilder;
pub use self::all_runners::AllRunnersBuilderError;

pub use self::edit::EditRunner;
pub use self::edit::EditRunnerBuilder;
pub use self::edit::EditRunnerBuilderError;
pub use self::edit::RunnerAccessLevel;

pub use self::runner::Runner;
pub use self::runner::RunnerBuilder;
pub use self::runner::RunnerBuilderError;

pub use self::runners::RunnerStatus;
pub use self::runners::RunnerType;
pub use self::runners::Runners;
pub use self::runners::RunnersBuilder;
pub use self::runners::RunnersBuilderError;
