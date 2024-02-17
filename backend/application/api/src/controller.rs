use std::str::FromStr;

use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use generated::{
    models::{self, AllVocabularyResponse, RecentlyVocabularyResponse, Vocabulary},
    VocabularyAllGetResponse, VocabularyIdDeleteResponse, VocabularyPostResponse,
    VocabularyPutResponse, VocabularyRecentGetResponse,
};
use mysql::{params, prelude::Queryable, Pool};

use crate::validatable::Validatable;

#[derive(Clone)]
pub struct Api {
    pub db: Pool,
}

impl AsRef<Api> for Api {
    #[inline]
    fn as_ref(&self) -> &Api {
        self
    }
}

#[async_trait]
impl generated::Api for Api {
    async fn vocabulary_recent_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<VocabularyRecentGetResponse, String> {
        let mut conn = self.db.get_conn().unwrap();

        let result = conn.query_map(
            "SELECT id, spelling, meaning, part_of_speech FROM vocabulary ORDER BY id DESC LIMIT 1;",
            |(id, spelling, meaning, part_of_speech): (i32, String, String, String)| Vocabulary {
                id: Some(id),
                part_of_speech: generated::models::PartOfSpeech::from_str(&part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
            },
        );

        match result {
            Ok(value) => {
                if let Some(vocabulary) = value.get(0) {
                    Ok(VocabularyRecentGetResponse::Status200_OkResponse(
                        RecentlyVocabularyResponse {
                            vocabulary: Vocabulary {
                                id: vocabulary.id,
                                part_of_speech: vocabulary.part_of_speech.clone(),
                                spelling: vocabulary.spelling.clone(),
                                meaning: vocabulary.meaning.clone(),
                            },
                        },
                    ))
                } else {
                    Ok(
                        VocabularyRecentGetResponse::Status404_TheSpecifiedResourceWasNotFound(
                            models::Error { message: None },
                        ),
                    )
                }
            }
            Err(e) => {
                tracing::error!(
                    "Failed to execute the query: {} at {}/#L{}",
                    e,
                    std::module_path!(),
                    line!()
                );

                Ok(VocabularyRecentGetResponse::Status500_InternalServerError)
            }
        }
    }

    async fn vocabulary_all_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<VocabularyAllGetResponse, String> {
        let mut conn = self.db.get_conn().unwrap();

        let result = conn.query_map(
            "SELECT id, spelling, meaning, part_of_speech FROM vocabulary;",
            |(id, spelling, meaning, part_of_speech): (i32, String, String, String)| Vocabulary {
                id: Some(id),
                part_of_speech: generated::models::PartOfSpeech::from_str(&part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
            },
        );

        match result {
            Ok(list) => Ok(VocabularyAllGetResponse::Status200_OkResponse(
                AllVocabularyResponse {
                    vocabulary_list: list.clone(),
                    total_count: list.len() as i32,
                },
            )),
            Err(e) => {
                tracing::error!(
                    "Failed to execute the query: {} at {}/#L{}",
                    e,
                    std::module_path!(),
                    line!()
                );

                Ok(VocabularyAllGetResponse::Status500_InternalServerError)
            }
        }
    }

    async fn vocabulary_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::RegisterVocabularyRequestBody>,
    ) -> Result<VocabularyPostResponse, String> {
        let body = match body {
            Some(body) => body,
            None => {
                return Ok(VocabularyPostResponse::Status400_BadRequest(
                    generated::models::Error { message: None },
                ));
            }
        };

        let mut conn = self.db.get_conn().unwrap();

        let vocabulary = Vocabulary {
            id: None,
            part_of_speech: body.vocabulary.part_of_speech,
            spelling: body.vocabulary.spelling,
            meaning: body.vocabulary.meaning,
        };

        match vocabulary.validate() {
            true => {}
            false => {
                return Ok(VocabularyPostResponse::Status400_BadRequest(
                    generated::models::Error { message: None },
                ));
            }
        }

        match conn.exec_drop(
            r"INSERT INTO vocabulary (spelling, meaning, part_of_speech)
                VALUES(:spelling, :meaning, :part_of_speech)",
            params! {
                "spelling" => vocabulary.spelling,
                "meaning" => vocabulary.meaning,
                "part_of_speech" => vocabulary.part_of_speech.to_string(),
            },
        ) {
            Ok(_) => Ok(VocabularyPostResponse::Status201_OkResponse(
                models::RegisterVocabularyOkResponse {
                    message: "Resource registered successfully".to_string(),
                },
            )),
            Err(e) => {
                Ok(VocabularyPostResponse::Status500_InternalServerError)
            },
        }
    }

    async fn vocabulary_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::UpdateVocabularyRequestBody>,
    ) -> Result<VocabularyPutResponse, String> {
        let body = match body {
            Some(body) => body,
            None => {
                return Ok(VocabularyPutResponse::Status400_BadRequest(
                    generated::models::Error { message: None },
                ));
            }
        };

        let mut conn = self.db.get_conn().unwrap();

        let vocabulary = Vocabulary {
            meaning: body.vocabulary.meaning,
            id: Some(body.vocabulary.id),
            part_of_speech: body.vocabulary.part_of_speech,
            spelling: body.vocabulary.spelling,
        };

        match vocabulary.validate() {
            true => {}
            false => {
                return Ok(VocabularyPutResponse::Status400_BadRequest(
                    generated::models::Error { message: None },
                ));
            }
        }

        match conn.exec_drop(
            r"UPDATE vocabulary SET spelling=(:spelling), meaning=(:meaning), part_of_speech=(:part_of_speech), updated_at=CURTIME() WHERE id=(:id);",
            params! {
                "spelling" => vocabulary.spelling,
                "meaning" => vocabulary.meaning,
                "part_of_speech" => vocabulary.part_of_speech.to_string(),
                "id" => vocabulary.id,
            },
        ) {
            Ok(_) => Ok(VocabularyPutResponse::Status200_OkResponse(
                models::UpdateVocabularyOkResponse {
                    message: "Resource updated successfully".to_string(),
                },
            )),
            Err(e) => {
                tracing::error!(
                    "Failed to execute the query: {} at {}/#L{}",
                    e,
                    std::module_path!(),
                    line!()
                );
                
                Ok(VocabularyPutResponse::Status500_InternalServerError)
            },
        }
    }

    async fn vocabulary_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::VocabularyIdDeletePathParams,
    ) -> Result<VocabularyIdDeleteResponse, String> {
        let mut conn = self.db.get_conn().unwrap();

        match conn.exec_drop(
            r"DELETE FROM vocabulary WHERE id=(:id);",
            params! {
                "id" => path_params.id
            },
        ) {
            Ok(_) => Ok(VocabularyIdDeleteResponse::Status200_OkResponse(
                models::DeleteVocabularyOkResponse {
                    message: "Resource deleted successfully".to_string(),
                },
            )),
            Err(e) => {
                tracing::error!(
                    "Failed to execute the query: {} at {}/#L{}",
                    e,
                    std::module_path!(),
                    line!()
                );

                Ok(VocabularyIdDeleteResponse::Status500_InternalServerError)
            },
        }
    }
}
