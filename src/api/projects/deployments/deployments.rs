// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::projects::deployments::DeploymentStatus;
use crate::api::ParamValue;

/// Sort orderings for deployments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeploymentOrderBy {
    Id,
    Iid,
    CreatedAt,
    UpdatedAt,
    FinishedAt,
    Ref,
}

impl DeploymentOrderBy {
    /// The scope as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Iid => "iid",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
            Self::FinishedAt => "finished_at",
            Self::Ref => "ref",
        }
    }
}

impl ParamValue<'static> for DeploymentOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Statuses deployments can be filtered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeploymentStatusFilter {
    Created,
    Running,
    Success,
    Failed,
    Canceled,
    Blocked,
}

impl DeploymentStatusFilter {
    /// The status as a query parameter.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
            Self::Blocked => "blocked",
        }
    }
}

impl From<DeploymentStatus> for DeploymentStatusFilter {
    fn from(status: DeploymentStatus) -> Self {
        match status {
            DeploymentStatus::Running => Self::Running,
            DeploymentStatus::Success => Self::Success,
            DeploymentStatus::Failed => Self::Failed,
            DeploymentStatus::Canceled => Self::Canceled,
        }
    }
}

impl ParamValue<'static> for DeploymentStatusFilter {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for deployments within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Deployments<'a> {
    /// The project to query for deployments.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The order to use for returned results.
    #[builder(default)]
    order_by: Option<DeploymentOrderBy>,
    /// The sort direction for returned results.
    #[builder(default)]
    sort: Option<SortOrder>,
    /// Only include deployments updated after a date.
    #[builder(default)]
    updated_after: Option<DateTime<Utc>>,
    /// Only include deployments updated before a date.
    #[builder(default)]
    updated_before: Option<DateTime<Utc>>,
    /// Only include deployments finished after a date.
    #[builder(default)]
    finished_after: Option<DateTime<Utc>>,
    /// Only include deployments finished before a date.
    #[builder(default)]
    finished_before: Option<DateTime<Utc>>,
    /// Only include deployments to a named environment.
    #[builder(setter(into), default)]
    environment: Option<Cow<'a, str>>,
    /// Only include deployments of a specific status.
    #[builder(setter(into), default)]
    status: Option<DeploymentStatusFilter>,
}

impl<'a> Deployments<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeploymentsBuilder<'a> {
        DeploymentsBuilder::default()
    }
}

impl<'a> Endpoint for Deployments<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/deployments", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort)
            .push_opt("updated_after", self.updated_after)
            .push_opt("updated_before", self.updated_before)
            .push_opt("finished_after", self.finished_after)
            .push_opt("finished_before", self.finished_before)
            .push_opt("environment", self.environment.as_ref())
            .push_opt("status", self.status);

        params
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::common::SortOrder;
    use crate::api::projects::deployments::{
        DeploymentOrderBy, DeploymentStatus, DeploymentStatusFilter, Deployments,
        DeploymentsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn deployment_order_by_as_str() {
        let items = &[
            (DeploymentOrderBy::Id, "id"),
            (DeploymentOrderBy::Iid, "iid"),
            (DeploymentOrderBy::CreatedAt, "created_at"),
            (DeploymentOrderBy::UpdatedAt, "updated_at"),
            (DeploymentOrderBy::FinishedAt, "finished_at"),
            (DeploymentOrderBy::Ref, "ref"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn deployment_status_filter_as_str() {
        let items = &[
            (DeploymentStatusFilter::Created, "created"),
            (DeploymentStatusFilter::Running, "running"),
            (DeploymentStatusFilter::Success, "success"),
            (DeploymentStatusFilter::Failed, "failed"),
            (DeploymentStatusFilter::Canceled, "canceled"),
            (DeploymentStatusFilter::Blocked, "blocked"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn deployment_status_filter_from_deployment_status() {
        let items = &[
            (DeploymentStatus::Running, DeploymentStatusFilter::Running),
            (DeploymentStatus::Success, DeploymentStatusFilter::Success),
            (DeploymentStatus::Failed, DeploymentStatusFilter::Failed),
            (DeploymentStatus::Canceled, DeploymentStatusFilter::Canceled),
        ];

        for (i, f) in items {
            assert_eq!(DeploymentStatusFilter::from(*i), *f);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = Deployments::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeploymentsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Deployments::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/deployments")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("order_by", "ref")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .order_by(DeploymentOrderBy::Ref)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("sort", "desc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .sort(SortOrder::Descending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("updated_after", "2024-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .updated_after(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_updated_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("updated_before", "2024-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .updated_before(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_finished_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("finished_after", "2024-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .finished_after(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_finished_before() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("finished_before", "2024-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .finished_before(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_environment() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("environment", "env")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .environment("env")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/deployments")
            .add_query_params(&[("status", "failed")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Deployments::builder()
            .project(1)
            .status(DeploymentStatus::Failed)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
