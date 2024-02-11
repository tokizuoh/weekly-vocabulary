use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use generated::GetRecentGetResponse;

pub struct Api {}

#[async_trait]
impl generated::Api for Api {
    async fn get_recent_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetRecentGetResponse, String> {
        // TODO
        return Ok(GetRecentGetResponse::Status404_NotFound);
    }
}
