use crate::app_state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mysql::{prelude::Queryable, *};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Adjective,
    Verb,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
}

impl PartOfSpeech {
    pub fn text(&self) -> &str {
        match self {
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Pronoun => "pronoun",
            PartOfSpeech::Adjective => "adjectiv",
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Adverb => "adverb",
            PartOfSpeech::Preposition => "preposition",
            PartOfSpeech::Conjunction => "conjunction",
            PartOfSpeech::Interjection => "interjection",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    let result = conn
        .query_map(
            "SELECT spelling, meaning FROM vocabulary_list ORDER BY id DESC LIMIT 1;",
            |(spelling, meaning)| {
                Vocabulary {
                    part_of_speech: PartOfSpeech::Verb, // TODO
                    spelling: spelling,
                    meaning: meaning,
                }
            },
        )
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    if let Some(vocabulary) = result.get(0) {
        let json_response = serde_json::json!({
            "part_of_speech": vocabulary.part_of_speech.text(),
            "spelling": vocabulary.spelling,
            "meaning": vocabulary.meaning,
        });

        Ok(Json(json_response))
    } else {
        // TODO: empty
        let json_response = serde_json::json!({
            "status": "ok",
            "message": "Empty",
        });

        Err((StatusCode::OK, Json(json_response)))
    }
}

pub async fn get_all_vocabulary(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    let result = conn
        .query_map(
            "SELECT spelling, meaning FROM vocabulary_list;",
            |(spelling, meaning)| {
                Vocabulary {
                    part_of_speech: PartOfSpeech::Verb, // TODO
                    spelling: spelling,
                    meaning: meaning,
                }
            },
        )
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
            "vocabulary_list": result,
    });

    Ok(Json(json_response))
}
