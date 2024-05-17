// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Personal access token-related API endpoints
//!
//! These endpoints are used for querying and modifying personal access tokens.

mod personal_access_token;
mod personal_access_token_self;
mod personal_access_tokens;
mod revoke;
mod revoke_self;
mod rotate;
mod rotate_self;

pub use self::personal_access_token::PersonalAccessToken;
pub use self::personal_access_token::PersonalAccessTokenBuilder;
pub use self::personal_access_token::PersonalAccessTokenBuilderError;

pub use self::personal_access_token_self::PersonalAccessTokenSelf;
pub use self::personal_access_token_self::PersonalAccessTokenSelfBuilder;
pub use self::personal_access_token_self::PersonalAccessTokenSelfBuilderError;

pub use self::personal_access_tokens::PersonalAccessTokenState;
pub use self::personal_access_tokens::PersonalAccessTokens;
pub use self::personal_access_tokens::PersonalAccessTokensBuilder;
pub use self::personal_access_tokens::PersonalAccessTokensBuilderError;

pub use self::revoke::RevokePersonalAccessToken;
pub use self::revoke::RevokePersonalAccessTokenBuilder;
pub use self::revoke::RevokePersonalAccessTokenBuilderError;

pub use self::revoke_self::RevokePersonalAccessTokenSelf;
pub use self::revoke_self::RevokePersonalAccessTokenSelfBuilder;
pub use self::revoke_self::RevokePersonalAccessTokenSelfBuilderError;

pub use self::rotate::RotatePersonalAccessToken;
pub use self::rotate::RotatePersonalAccessTokenBuilder;
pub use self::rotate::RotatePersonalAccessTokenBuilderError;

pub use self::rotate_self::RotatePersonalAccessTokenSelf;
pub use self::rotate_self::RotatePersonalAccessTokenSelfBuilder;
pub use self::rotate_self::RotatePersonalAccessTokenSelfBuilderError;
