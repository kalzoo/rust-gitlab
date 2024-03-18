// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query for deploy keys.
#[derive(Debug, Builder, Clone)]
pub struct Job {}

impl Job {
    /// Create a builder for the endpoint.
    pub fn builder() -> JobBuilder {
        JobBuilder::default()
    }
}

impl Endpoint for Job {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "job".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::job::Job;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn default_is_sufficient() {
        Job::builder().build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("job").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Job::builder().build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
