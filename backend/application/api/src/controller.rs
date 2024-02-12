use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use generated::{
    models::{self, RecentlyVocabularyResponse, Vocabulary},
    GetRecentGetResponse,
};
use mysql::{prelude::Queryable, Pool};

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
}
