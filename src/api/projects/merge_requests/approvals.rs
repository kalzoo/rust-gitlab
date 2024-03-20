// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project merge request approvals API endpoint.
//!
//! This endpoint is used for querying project merge request approvals.
//! See <https://docs.gitlab.com/ee/api/merge_request_approvals.html#merge-request-level-mr-approvals>
//!
//! # Example
//! ```rust,no_run
//! use serde::Deserialize;
//! use gitlab::Gitlab;
//! use gitlab::api::{self, Query};
//! use chrono::{DateTime, Utc};
//!
//! #[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum MergeRequestState {
//!     #[serde(rename = "opened")]
//!     Opened,
//!     #[serde(rename = "closed")]
//!     Closed,
//!     #[serde(rename = "reopened")]
//!     Reopened,
//!     #[serde(rename = "merged")]
//!     Merged,
//!     #[serde(rename = "locked")]
//!     Locked,
//! }
//!
//! #[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum MergeStatus {
//!     #[serde(rename = "preparing")]
//!     Preparing,
//!     #[serde(rename = "unchecked")]
//!     Unchecked,
//!     #[serde(rename = "checking")]
//!     Checking,
//!     #[serde(rename = "can_be_merged")]
//!     CanBeMerged,
//!     #[serde(rename = "cannot_be_merged")]
//!     CannotBeMerged,
//!     #[serde(rename = "cannot_be_merged_recheck")]
//!     CannotBeMergedRecheck,
//!     #[serde(rename = "cannot_be_merged_rechecking")]
//!     CannotBeMergedRechecking,
//! }
//!
//! #[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum UserState {
//!     #[serde(rename = "active")]
//!     Active,
//!     #[serde(rename = "blocked")]
//!     Blocked,
//!     #[serde(rename = "ldap_blocked")]
//!     LdapBlocked,
//!     #[serde(rename = "deactivated")]
//!     Deactivated,
//! }
//!
//! #[derive(Deserialize, Debug, Clone)]
//! pub struct UserBasic {
//!     pub username: String,
//!     pub name: String,
//!     pub id: u64,
//!     pub state: UserState,
//! }
//!
//! #[derive(Deserialize, Debug, Clone)]
//! pub struct MergeRequestApprovals {
//!     pub id: u64,
//!     pub iid: u64,
//!     pub project_id: u64,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub state: MergeRequestState,
//!     pub created_at: DateTime<Utc>,
//!     pub updated_at: DateTime<Utc>,
//!     pub merge_status: MergeStatus,
//!     pub approvals_required: u64,
//!     pub approvals_left: u64,
//!     pub approved_by: Vec<UserBasic>,
//! }
//! // Create the client.
//! let client = Gitlab::new("gitlab.com", "private-token").unwrap();
//! // Create the endpoint for the merge request 34 in project 12.
//! let endpoint = api::projects::merge_requests::approvals::MergeRequestApprovals::builder()
//!     .project(12)
//!     .merge_request(34)
//!     .build()
//!     .unwrap();
//! // Get the merge request with approvals.
//! let approvals: MergeRequestApprovals = endpoint.query(&client).unwrap();
//! ```

mod approvals;

pub use self::approvals::MergeRequestApprovals;
pub use self::approvals::MergeRequestApprovalsBuilder;
pub use self::approvals::MergeRequestApprovalsBuilderError;
