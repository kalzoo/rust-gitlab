// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Personal access token endpoints
//!
//! Personal access token endpoints for users.

mod create;
mod create_for_user;

pub use self::create::CreatePersonalAccessToken;
pub use self::create::CreatePersonalAccessTokenBuilder;
pub use self::create::CreatePersonalAccessTokenBuilderError;
pub use self::create::PersonalAccessTokenScope;

pub use self::create_for_user::CreatePersonalAccessTokenForUser;
pub use self::create_for_user::CreatePersonalAccessTokenForUserBuilder;
pub use self::create_for_user::CreatePersonalAccessTokenForUserBuilderError;
