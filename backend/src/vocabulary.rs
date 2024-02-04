use crate::app_state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mysql::{prelude::Queryable, *};
use serde::{Deserialize, Serialize};
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

impl PartOfSpeech {
    fn from_string(s: String) -> std::prelude::v1::Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "noun" => Ok(PartOfSpeech::Noun),
            "pronoun" => Ok(PartOfSpeech::Pronoun),
            "adjective" => Ok(PartOfSpeech::Adjective),
            "verb" => Ok(PartOfSpeech::Verb),
            "adverb" => Ok(PartOfSpeech::Adverb),
            "preposition" => Ok(PartOfSpeech::Preposition),
            "conjunction" => Ok(PartOfSpeech::Conjunction),
            "interjection" => Ok(PartOfSpeech::Interjection),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}

#[derive(Debug, Deserialize)]
pub struct VocabularyInput {
    pub part_of_speech: String,
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    let result = conn
        .query_map(
            "SELECT spelling, meaning, part_of_speech FROM vocabulary_list ORDER BY id DESC LIMIT 1;",
            |(spelling, meaning, part_of_speech)| {
                Vocabulary {
                    part_of_speech: PartOfSpeech::from_string(part_of_speech).unwrap(),
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
            "data": {
                "part_of_speech": vocabulary.part_of_speech.text(),
                "spelling": vocabulary.spelling,
                "meaning": vocabulary.meaning,
            }
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
            "SELECT spelling, meaning, part_of_speech FROM vocabulary_list;",
            |(spelling, meaning, part_of_speech)| Vocabulary {
                part_of_speech: PartOfSpeech::from_string(part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
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
        "metadata": {
            "total_count": result.len(),
        },
        "data": result,
    });

    Ok(Json(json_response))
}

pub async fn register_vocabulary(
    State(data): State<Arc<AppState>>,
    Json(body): Json<VocabularyInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    match PartOfSpeech::from_string(body.part_of_speech.clone()) {
        Ok(_) => {}
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("invalid part_of_speech: {}", body.part_of_speech),
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    conn.exec_drop(
        r"INSERT INTO vocabulary_list (spelling, meaning, part_of_speech)
            VALUES(:spelling, :meaning, :part_of_speech)",
        params! {
            "spelling" => body.spelling,
            "meaning" => body.meaning,
            "part_of_speech" => body.part_of_speech.clone()
        },
    );

    // TODO: return payload
    let json_response = serde_json::json!({
        "ok": "ok"
    });

    Ok(Json(json_response))
}
