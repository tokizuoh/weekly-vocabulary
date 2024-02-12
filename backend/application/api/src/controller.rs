use crate::vocabulary::{PartOfSpeech, Vocabulary};
use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use generated::{models::GetRecentGet200Response, GetRecentGetResponse};
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

        let result = conn
        .query_map(
            "SELECT spelling, meaning, part_of_speech FROM vocabulary ORDER BY id DESC LIMIT 1;",
            |(spelling, meaning, part_of_speech)| Vocabulary {
                part_of_speech: PartOfSpeech::from_string(part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
            },
        )
        .map_err(|_e| {
            // TODO
            // let error_response = serde_json::json!({
                // "message": format!("Database error: {}", e),
            // });

            // Ok(GetRecentGetResponse::Status500_InternalServerError);
            ""
        });

        match result {
            Ok(value) => {
                if let Some(vocabulary) = value.get(0) {
                    Ok(
                        GetRecentGetResponse::Status200_GetRecentlyRegisiteredVocabulary(
                            GetRecentGet200Response {
                                part_of_speech: vocabulary.part_of_speech.text().to_string(),
                                spelling: vocabulary.spelling.clone(),
                                meaning: vocabulary.meaning.clone(),
                            },
                        ),
                    )
                } else {
                    Ok(GetRecentGetResponse::Status404_NotFound)
                }
            }
            Err(_) => Ok(GetRecentGetResponse::Status500_InternalServerError),
        }
    }
}
