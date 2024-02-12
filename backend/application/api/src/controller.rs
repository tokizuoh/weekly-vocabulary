use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use generated::{
    models::{self, AllVocabularyResponse, RecentlyVocabularyResponse, Vocabulary},
    GetAllGetResponse, GetRecentGetResponse, RegisterPutResponse,
};
use mysql::{params, prelude::Queryable, Pool};

use crate::vocabulary::Validatable;

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
    async fn get_recent_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetRecentGetResponse, String> {
        let mut conn = self.db.get_conn().unwrap();

        let result = conn.query_map(
            "SELECT id, spelling, meaning, part_of_speech FROM vocabulary ORDER BY id DESC LIMIT 1;",
            |(id, spelling, meaning, part_of_speech)| Vocabulary {
                id: id,
                part_of_speech: part_of_speech,
                spelling: spelling,
                meaning: meaning,
            },
        );

        match result {
            Ok(value) => {
                if let Some(vocabulary) = value.get(0) {
                    Ok(
                        GetRecentGetResponse::Status200_GetRecentlyRegisiteredVocabulary(
                            RecentlyVocabularyResponse {
                                vocabulary: Vocabulary {
                                    id: vocabulary.id,
                                    part_of_speech: vocabulary.part_of_speech.clone(),
                                    spelling: vocabulary.spelling.clone(),
                                    meaning: vocabulary.meaning.clone(),
                                },
                            },
                        ),
                    )
                } else {
                    Ok(
                        GetRecentGetResponse::Status404_TheSpecifiedResourceWasNotFound(
                            models::Error { message: None },
                        ),
                    )
                }
            }
            Err(e) => Ok(GetRecentGetResponse::Status500_InternalServerError(
                generated::models::Error {
                    message: Some(e.to_string()),
                },
            )),
        }
    }

    async fn get_all_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetAllGetResponse, String> {
        let mut conn = self.db.get_conn().unwrap();

        let result = conn.query_map(
            "SELECT id, spelling, meaning, part_of_speech FROM vocabulary;",
            |(id, spelling, meaning, part_of_speech)| Vocabulary {
                id: id,
                part_of_speech: part_of_speech,
                spelling: spelling,
                meaning: meaning,
            },
        );

        match result {
            Ok(list) => Ok(
                GetAllGetResponse::Status200_ReturnAllRegisiteredVocabularyList(
                    AllVocabularyResponse {
                        vocabulary_list: list.clone(),
                        total_count: list.len() as i32,
                    },
                ),
            ),
            Err(e) => Ok(GetAllGetResponse::Status500_InternalServerError(
                generated::models::Error {
                    message: Some(e.to_string()),
                },
            )),
        }
    }

    async fn register_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::RegisterVocabularyRequestBody>,
    ) -> Result<RegisterPutResponse, String> {
        let body = match body {
            Some(body) => body,
            None => {
                return Ok(RegisterPutResponse::Status400_BadRequest(
                    generated::models::Error { message: None },
                ));
            }
        };

        let mut conn = self.db.get_conn().unwrap();

        let vocabulary = Vocabulary::new(
            body.vocabulary.part_of_speech,
            body.vocabulary.spelling,
            body.vocabulary.meaning,
        );

        match vocabulary.validate() {
            true => {}
            false => {
                return Ok(RegisterPutResponse::Status400_BadRequest(
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
                "part_of_speech" => vocabulary.part_of_speech,
            },
        ) {
            Ok(_) => Ok(RegisterPutResponse::Status200_RegisiterVocabulary(
                models::RegisterVocabularyOkResponse {
                    message: "Resource updated successfully".to_string(),
                },
            )),
            Err(e) => Ok(RegisterPutResponse::Status500_InternalServerError(
                models::Error {
                    message: Some(format!("Failed to update vocabulary: {}", e)),
                },
            )),
        }
    }
}
