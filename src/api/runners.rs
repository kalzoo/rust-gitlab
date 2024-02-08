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
mod create;
mod delete;
mod delete_by_token;
mod edit;
mod reset_authentication_token;
mod reset_authentication_token_by_token;
mod runner;
mod runners;

pub use self::all_runners::AllRunners;
pub use self::all_runners::AllRunnersBuilder;
pub use self::all_runners::AllRunnersBuilderError;

pub use self::create::CreateRunner;
pub use self::create::CreateRunnerBuilder;
pub use self::create::CreateRunnerBuilderError;

pub use self::delete::DeleteRunner;
pub use self::delete::DeleteRunnerBuilder;
pub use self::delete::DeleteRunnerBuilderError;

pub use self::delete_by_token::DeleteRunnerByToken;
pub use self::delete_by_token::DeleteRunnerByTokenBuilder;
pub use self::delete_by_token::DeleteRunnerByTokenBuilderError;

pub use self::edit::EditRunner;
pub use self::edit::EditRunnerBuilder;
pub use self::edit::EditRunnerBuilderError;
pub use self::edit::RunnerAccessLevel;

pub use self::reset_authentication_token::ResetRunnerAuthenticationToken;
pub use self::reset_authentication_token::ResetRunnerAuthenticationTokenBuilder;
pub use self::reset_authentication_token::ResetRunnerAuthenticationTokenBuilderError;

pub use self::reset_authentication_token_by_token::ResetRunnerAuthenticationTokenByToken;
pub use self::reset_authentication_token_by_token::ResetRunnerAuthenticationTokenByTokenBuilder;
pub use self::reset_authentication_token_by_token::ResetRunnerAuthenticationTokenByTokenBuilderError;

pub use self::runner::Runner;
pub use self::runner::RunnerBuilder;
pub use self::runner::RunnerBuilderError;

pub use self::runners::RunnerStatus;
pub use self::runners::RunnerType;
pub use self::runners::Runners;
pub use self::runners::RunnersBuilder;
pub use self::runners::RunnersBuilderError;
