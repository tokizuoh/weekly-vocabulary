use std::ptr::null;

use axum::{http::StatusCode, response::IntoResponse, Json};
use dotenv::dotenv;
use mysql::{prelude::Queryable, *};
use serde_json::json;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Vocabulary {
    pub part_of_speech: PartOfSpeech,
    pub spelling: String,
    pub meaning: String,
}

pub async fn get_latest_vocabulary(
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // TODO: commonize .env and .env.dev
    dotenv::from_filename("./.env.dev").ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(url.as_str()).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let mut conn = pool.get_conn().unwrap();

    let result = conn
        .query_map(
            "SELECT spelling, meaning FROM vocabulary_list ORDER BY created_at ASC LIMIT 1;",
            // |(_, spelling, meaning, _, _, _): (_, String, String, String, _, _)| {
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
