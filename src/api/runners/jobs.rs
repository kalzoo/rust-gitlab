// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Status of the job on a runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RunnerJobStatus {
    /// The job is currently running.
    Running,
    /// The job succeeded.
    Success,
    /// The job failed.
    Failed,
    /// The job was canceled.
    Canceled,
}

impl RunnerJobStatus {
    /// The runner job status as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
        }
    }
}

impl ParamValue<'static> for RunnerJobStatus {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Keys runner job results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RunnerJobsOrderBy {
    /// Order by the ID of the job.
    Id,
}

impl RunnerJobsOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
        }
    }
}

impl ParamValue<'static> for RunnerJobsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for jobs on a runner.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct RunnerJobs<'a> {
    /// The ID of the runner.
    runner: u64,

    /// Filter by the system ID of the runner manager.
    #[builder(setter(into), default)]
    system_id: Option<Cow<'a, str>>,
    /// Filter jobs by job status.
    #[builder(default)]
    status: Option<RunnerJobStatus>,
    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<RunnerJobsOrderBy>,
    /// The sort order for returned results.
    #[builder(setter(into), default)]
    sort: Option<SortOrder>,
}

impl<'a> RunnerJobs<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RunnerJobsBuilder<'a> {
        RunnerJobsBuilder::default()
    }
}

impl<'a> Endpoint for RunnerJobs<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("runners/{}/jobs", self.runner).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("system_id", self.system_id.as_ref())
            .push_opt("status", self.status)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for RunnerJobs<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::SortOrder;
    use crate::api::runners::{
        RunnerJobStatus, RunnerJobs, RunnerJobsBuilderError, RunnerJobsOrderBy,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn runner_jobs_order_by_as_str() {
        let items = &[(RunnerJobsOrderBy::Id, "id")];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn runner_job_status_as_str() {
        let items = &[
            (RunnerJobStatus::Running, "running"),
            (RunnerJobStatus::Success, "success"),
            (RunnerJobStatus::Failed, "failed"),
            (RunnerJobStatus::Canceled, "canceled"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn runner_is_required() {
        let err = RunnerJobs::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, RunnerJobsBuilderError, "runner");
    }

    #[test]
    fn runner_is_sufficient() {
        RunnerJobs::builder().runner(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/1/jobs")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RunnerJobs::builder().runner(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_system_id() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/1/jobs")
            .add_query_params(&[("system_id", "system")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RunnerJobs::builder()
            .runner(1)
            .system_id("system")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_status() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/1/jobs")
            .add_query_params(&[("status", "failed")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RunnerJobs::builder()
            .runner(1)
            .status(RunnerJobStatus::Failed)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/1/jobs")
            .add_query_params(&[("order_by", "id")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RunnerJobs::builder()
            .runner(1)
            .order_by(RunnerJobsOrderBy::Id)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("runners/1/jobs")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RunnerJobs::builder()
            .runner(1)
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
