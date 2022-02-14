use crate::api::Endpoint;
use crate::api::{query, ApiError, AsyncClient};
use http::{header, Request};
use serde::de::DeserializeOwned;

pub struct AsyncQueryWithResponseReturnValue<T> {
    pub body: T,
    pub response: http::Response<bytes::Bytes>,
}

pub async fn query_async_with_response<E, T, C>(
    endpoint: &E,
    client: &C,
) -> Result<AsyncQueryWithResponseReturnValue<T>, ApiError<C::Error>>
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    let mut url = client.rest_endpoint(&endpoint.endpoint())?;
    endpoint.parameters().add_to_url(&mut url);

    let req = Request::builder()
        .method(endpoint.method())
        .uri(query::url_to_http_uri(url));
    let (req, data) = if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        (req, data)
    } else {
        (req, Vec::new())
    };
    let rsp = client.rest_async(req, data).await?;
    let status = rsp.status();
    let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
        v
    } else {
        return Err(ApiError::server_error(status, rsp.body()));
    };
    if !status.is_success() {
        return Err(ApiError::from_gitlab(v));
    }

    let body = serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)?;

    Ok(AsyncQueryWithResponseReturnValue {
        body,
        response: rsp,
    })
}

pub struct AsyncQueryRawWithResponseReturnValue {
    pub response: http::Response<bytes::Bytes>,
}

impl AsyncQueryRawWithResponseReturnValue {
    pub fn get_body(self) -> Vec<u8> {
        self.response.into_body().as_ref().into()
    }
}

pub async fn query_async_raw_with_response<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<AsyncQueryRawWithResponseReturnValue, ApiError<C::Error>>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    let mut url = client.rest_endpoint(&endpoint.endpoint())?;
    endpoint.parameters().add_to_url(&mut url);

    let req = Request::builder()
        .method(endpoint.method())
        .uri(query::url_to_http_uri(url));
    let (req, data) = if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        (req, data)
    } else {
        (req, Vec::new())
    };
    let rsp = client.rest_async(req, data).await?;
    if !rsp.status().is_success() {
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(rsp.status(), rsp.body()));
        };
        return Err(ApiError::from_gitlab(v));
    }

    Ok(AsyncQueryRawWithResponseReturnValue { response: rsp })
}
