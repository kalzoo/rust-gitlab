// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Issue API endpoints and types.
//!
//! These endpoints are used for querying issues from projects, groups or the whole instance.

use std::collections::BTreeSet;

use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

pub use groups::{GroupIssues, GroupIssuesBuilder, GroupIssuesBuilderError};
pub use projects::{ProjectIssues, ProjectIssuesBuilder, ProjectIssuesBuilderError};

mod groups;
mod projects;

/// Filters for issue states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueState {
    /// Filter issues that are open.
    Opened,
    /// Filter issues that are closed.
    Closed,
}

impl IssueState {
    fn as_str(self) -> &'static str {
        match self {
            IssueState::Opened => "opened",
            IssueState::Closed => "closed",
        }
    }
}

impl ParamValue<'static> for IssueState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filter issues by a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueScope {
    /// Filter issues created by the API caller.
    CreatedByMe,
    /// Filter issues assigned to the API caller.
    AssignedToMe,
    /// Return all issues.
    All,
}

impl IssueScope {
    fn as_str(self) -> &'static str {
        match self {
            IssueScope::CreatedByMe => "created_by_me",
            IssueScope::AssignedToMe => "assigned_to_me",
            IssueScope::All => "all",
        }
    }
}

impl ParamValue<'static> for IssueScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Types of issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueType {
    /// Regular issues.
    Issue,
    /// Incident reports.
    Incident,
    /// Test case issues.
    TestCase,
}

impl IssueType {
    fn as_str(self) -> &'static str {
        match self {
            IssueType::Issue => "issue",
            IssueType::Incident => "incident",
            IssueType::TestCase => "test_case",
        }
    }
}

impl ParamValue<'static> for IssueType {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filter values by epic status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueEpic {
    /// Issues without any epic.
    None,
    /// Issues with any epic association.
    Any,
    /// Issues with a given epic (by ID).
    Id(u64),
}

impl IssueEpic {
    fn as_str(self) -> Cow<'static, str> {
        match self {
            IssueEpic::None => "None".into(),
            IssueEpic::Any => "Any".into(),
            IssueEpic::Id(id) => format!("{}", id).into(),
        }
    }
}

impl From<u64> for IssueEpic {
    fn from(id: u64) -> Self {
        Self::Id(id)
    }
}

impl ParamValue<'static> for IssueEpic {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str()
    }
}

/// Health statuses of issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueHealthStatus {
    /// Issues with any health status.
    Any,
    /// Issues without a health status.
    None,
    /// Issues that are on track.
    OnTrack,
    /// Issues that need attention.
    NeedsAttention,
    /// Issues that are at risk.
    AtRisk,
}

impl IssueHealthStatus {
    fn as_str(self) -> &'static str {
        match self {
            IssueHealthStatus::Any => "Any",
            IssueHealthStatus::None => "None",
            IssueHealthStatus::OnTrack => "on_track",
            IssueHealthStatus::NeedsAttention => "needs_attention",
            IssueHealthStatus::AtRisk => "at_risk",
        }
    }
}

impl ParamValue<'static> for IssueHealthStatus {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filter values for issue iteration values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueIteration<'a> {
    /// Issues without any iteration.
    None,
    /// Issues with any iteration association.
    Any,
    /// Issues with a given iteration (by ID).
    Id(u64),
    /// Issues with a tiven iteration (by title).
    Title(Cow<'a, str>),
}

impl<'a> IssueIteration<'a> {
    fn add_params<'b>(&'b self, params: &mut QueryParams<'b>) {
        match self {
            IssueIteration::None => {
                params.push("iteration_id", "None");
            },
            IssueIteration::Any => {
                params.push("iteration_id", "Any");
            },
            IssueIteration::Id(id) => {
                params.push("iteration_id", *id);
            },
            IssueIteration::Title(title) => {
                params.push("iteration_title", title);
            },
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
enum Assignee<'a> {
    Assigned,
    Unassigned,
    Id(u64),
    Usernames(BTreeSet<Cow<'a, str>>),
}

impl<'a> Assignee<'a> {
    fn add_params<'b>(&'b self, params: &mut QueryParams<'b>) {
        match self {
            Assignee::Assigned => {
                params.push("assignee_id", "Any");
            },
            Assignee::Unassigned => {
                params.push("assignee_id", "None");
            },
            Assignee::Id(id) => {
                params.push("assignee_id", *id);
            },
            Assignee::Usernames(usernames) => {
                params.extend(usernames.iter().map(|value| ("assignee_username[]", value)));
            },
        }
    }
}

/// Filter issues by weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueWeight {
    /// Filter issues with any weight.
    Any,
    /// Filter issues with no weight assigned.
    None,
    /// Filter issues with a specific weight.
    Weight(u64),
}

impl IssueWeight {
    fn as_str(self) -> Cow<'static, str> {
        match self {
            IssueWeight::Any => "Any".into(),
            IssueWeight::None => "None".into(),
            IssueWeight::Weight(weight) => weight.to_string().into(),
        }
    }
}

impl ParamValue<'static> for IssueWeight {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str()
    }
}

/// The scope to apply search query terms to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueSearchScope {
    /// Search within titles.
    Title,
    /// Search within descriptions.
    Description,
}

impl IssueSearchScope {
    fn as_str(self) -> &'static str {
        match self {
            IssueSearchScope::Title => "title",
            IssueSearchScope::Description => "description",
        }
    }
}

impl ParamValue<'static> for IssueSearchScope {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filter values for due dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueDueDateFilter {
    /// Issues without a due date.
    None,
    /// Issues with any a due date.
    Any,
    /// Issues due today.
    Today,
    /// Issues due tomorrow.
    Tomorrow,
    /// Issues due this week.
    ThisWeek,
    /// Issues due this month.
    ThisMonth,
    /// Issues due between two weeks ago and a month from now.
    BetweenTwoWeeksAgoAndNextMonth,
    /// Issues which are overdue.
    Overdue,
}

impl IssueDueDateFilter {
    fn as_str(self) -> &'static str {
        match self {
            IssueDueDateFilter::None => "0",
            IssueDueDateFilter::Any => "any",
            IssueDueDateFilter::Today => "today",
            IssueDueDateFilter::Tomorrow => "tomorrow",
            IssueDueDateFilter::ThisWeek => "week",
            IssueDueDateFilter::ThisMonth => "month",
            IssueDueDateFilter::BetweenTwoWeeksAgoAndNextMonth => {
                "next_month_and_previous_two_weeks"
            },
            IssueDueDateFilter::Overdue => "overdue",
        }
    }
}

impl ParamValue<'static> for IssueDueDateFilter {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Keys issue results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueOrderBy {
    /// Sort by creation date.
    CreatedAt,
    /// Sort by last updated date.
    UpdatedAt,
    /// Sort by priority.
    Priority,
    /// Sort by due date.
    DueDate,
    /// Sort by relative position.
    ///
    /// TOOD: position within what?
    RelativePosition,
    /// Sort by priority labels.
    LabelPriority,
    /// Sort by milestone due date.
    MilestoneDue,
    /// Sort by popularity.
    Popularity,
    /// Sort by weight.
    Weight,
    /// Sort by type.
    Title,
    /// Sort by weight.
    #[deprecated(note = "use `Weight` instead (`gitlab` crate typo)")]
    WeightFields,
}

#[allow(clippy::derivable_impls)]
impl Default for IssueOrderBy {
    fn default() -> Self {
        // XXX(rust-1.62): use `#[default]`
        IssueOrderBy::CreatedAt
    }
}

impl IssueOrderBy {
    fn as_str(self) -> &'static str {
        match self {
            IssueOrderBy::CreatedAt => "created_at",
            IssueOrderBy::UpdatedAt => "updated_at",
            IssueOrderBy::Priority => "priority",
            IssueOrderBy::DueDate => "due_date",
            IssueOrderBy::RelativePosition => "relative_position",
            IssueOrderBy::LabelPriority => "label_priority",
            IssueOrderBy::MilestoneDue => "milestone_due",
            IssueOrderBy::Popularity => "popularity",
            IssueOrderBy::Title => "title",
            #[allow(deprecated)]
            IssueOrderBy::Weight | IssueOrderBy::WeightFields => "weight",
        }
    }
}

impl ParamValue<'static> for IssueOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Filters available for issue milestones.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum IssueMilestone<'a> {
    /// Issues without any milestone.
    None,
    /// Issues with any milestone.
    Any,
    /// Issues with milestones with upcoming due dates.
    Upcoming,
    /// Issues with milestones that have started.
    Started,
    /// Issues with a specific milestone.
    Named(Cow<'a, str>),
}

impl<'a> IssueMilestone<'a> {
    fn as_str(&self) -> &str {
        match self {
            IssueMilestone::None => "None",
            IssueMilestone::Any => "Any",
            IssueMilestone::Upcoming => "Upcoming",
            IssueMilestone::Started => "Started",
            IssueMilestone::Named(name) => name.as_ref(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b IssueMilestone<'a> {
    fn as_value(&self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::issues::{
        IssueDueDateFilter, IssueEpic, IssueHealthStatus, IssueMilestone, IssueOrderBy, IssueScope,
        IssueSearchScope, IssueState, IssueType, IssueWeight,
    };

    #[test]
    fn issue_state_as_str() {
        let items = &[
            (IssueState::Opened, "opened"),
            (IssueState::Closed, "closed"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_scope_as_str() {
        let items = &[
            (IssueScope::CreatedByMe, "created_by_me"),
            (IssueScope::AssignedToMe, "assigned_to_me"),
            (IssueScope::All, "all"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_epic_from_u64() {
        let items = &[(IssueEpic::Id(4), 4.into())];

        for (i, s) in items {
            assert_eq!(i, s);
        }
    }

    #[test]
    fn issue_epic_as_str() {
        let items = &[
            (IssueEpic::None, "None"),
            (IssueEpic::Any, "Any"),
            (IssueEpic::Id(4), "4"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_health_status_as_str() {
        let items = &[
            (IssueHealthStatus::OnTrack, "on_track"),
            (IssueHealthStatus::NeedsAttention, "needs_attention"),
            (IssueHealthStatus::AtRisk, "at_risk"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_type_as_str() {
        let items = &[
            (IssueType::Issue, "issue"),
            (IssueType::Incident, "incident"),
            (IssueType::TestCase, "test_case"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_weight_as_str() {
        let items = &[
            (IssueWeight::Any, "Any"),
            (IssueWeight::None, "None"),
            (IssueWeight::Weight(0), "0"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_search_scope_as_str() {
        let items = &[
            (IssueSearchScope::Title, "title"),
            (IssueSearchScope::Description, "description"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_due_date_filter_as_str() {
        let items = &[
            (IssueDueDateFilter::None, "0"),
            (IssueDueDateFilter::Any, "any"),
            (IssueDueDateFilter::Today, "today"),
            (IssueDueDateFilter::Tomorrow, "tomorrow"),
            (IssueDueDateFilter::ThisWeek, "week"),
            (IssueDueDateFilter::ThisMonth, "month"),
            (
                IssueDueDateFilter::BetweenTwoWeeksAgoAndNextMonth,
                "next_month_and_previous_two_weeks",
            ),
            (IssueDueDateFilter::Overdue, "overdue"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_order_by_default() {
        assert_eq!(IssueOrderBy::default(), IssueOrderBy::CreatedAt);
    }

    #[test]
    fn issue_order_by_as_str() {
        let items = &[
            (IssueOrderBy::CreatedAt, "created_at"),
            (IssueOrderBy::UpdatedAt, "updated_at"),
            (IssueOrderBy::Priority, "priority"),
            (IssueOrderBy::DueDate, "due_date"),
            (IssueOrderBy::RelativePosition, "relative_position"),
            (IssueOrderBy::LabelPriority, "label_priority"),
            (IssueOrderBy::MilestoneDue, "milestone_due"),
            (IssueOrderBy::Popularity, "popularity"),
            (IssueOrderBy::Weight, "weight"),
            (IssueOrderBy::Title, "title"),
            #[allow(deprecated)]
            (IssueOrderBy::WeightFields, "weight"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_milestone_as_str() {
        let items = &[
            (IssueMilestone::Any, "Any"),
            (IssueMilestone::None, "None"),
            (IssueMilestone::Upcoming, "Upcoming"),
            (IssueMilestone::Started, "Started"),
            (IssueMilestone::Named("milestone".into()), "milestone"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }
}
