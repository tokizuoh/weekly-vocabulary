#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteDeleteResponse {
    /// ok response
    Status200_OkResponse
    (models::DeleteVocabularyOkResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::Error)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::Error)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAllGetResponse {
    /// ok response
    Status200_OkResponse
    (models::AllVocabularyResponse)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::Error)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetRecentGetResponse {
    /// ok response
    Status200_OkResponse
    (models::RecentlyVocabularyResponse)
    ,
    /// The specified resource was not found
    Status404_TheSpecifiedResourceWasNotFound
    (models::Error)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::Error)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RegisterPostResponse {
    /// ok response
    Status200_OkResponse
    (models::RegisterVocabularyOkResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::Error)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::Error)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePutResponse {
    /// ok response
    Status200_OkResponse
    (models::UpdateVocabularyOkResponse)
    ,
    /// Bad Request
    Status400_BadRequest
    (models::Error)
    ,
    /// Internal Server Error
    Status500_InternalServerError
    (models::Error)
}


/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {

                /// DeleteDelete - DELETE /delete
                async fn delete_delete(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: Option<models::DeleteVocabularyRequestBody>,
                ) -> Result<DeleteDeleteResponse, String>;


                /// GetAllGet - GET /get/all
                async fn get_all_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<GetAllGetResponse, String>;


                /// GetRecentGet - GET /get/recent
                async fn get_recent_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<GetRecentGetResponse, String>;


                /// RegisterPost - POST /register
                async fn register_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: Option<models::RegisterVocabularyRequestBody>,
                ) -> Result<RegisterPostResponse, String>;


                /// UpdatePut - PUT /update
                async fn update_put(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: Option<models::UpdateVocabularyRequestBody>,
                ) -> Result<UpdatePutResponse, String>;

}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
