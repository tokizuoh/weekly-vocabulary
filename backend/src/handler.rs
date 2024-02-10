use crate::app_state::AppState;
use crate::vocabulary::{PartOfSpeech, Vocabulary};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mysql::{prelude::Queryable, *};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct RegisterVocabularyInput {
    pub part_of_speech: String,
    pub spelling: String,
    pub meaning: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteVocabularyInput {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVocabularyInput {
    pub id: String,
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
            "SELECT spelling, meaning, part_of_speech FROM vocabulary ORDER BY id DESC LIMIT 1;",
            |(spelling, meaning, part_of_speech)| Vocabulary {
                part_of_speech: PartOfSpeech::from_string(part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
            },
        )
        .map_err(|e| {
            let error_response = serde_json::json!({
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
        let json_response = serde_json::json!({
            "message": "Resource not found",
        });

        Err((StatusCode::NOT_FOUND, Json(json_response)))
    }
}

pub async fn get_all_vocabulary(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    let result = conn
        .query_map(
            "SELECT spelling, meaning, part_of_speech FROM vocabulary;",
            |(spelling, meaning, part_of_speech)| Vocabulary {
                part_of_speech: PartOfSpeech::from_string(part_of_speech).unwrap(),
                spelling: spelling,
                meaning: meaning,
            },
        )
        .map_err(|e| {
            let error_response = serde_json::json!({
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
    Json(body): Json<RegisterVocabularyInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    match PartOfSpeech::from_string(body.part_of_speech.clone()) {
        Ok(_) => {}
        Err(_) => {
            let error_response = serde_json::json!({
                "message": format!("Invalid part_of_speech: {}", body.part_of_speech),
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    match conn.exec_drop(
        r"INSERT INTO vocabulary (spelling, meaning, part_of_speech)
            VALUES(:spelling, :meaning, :part_of_speech)",
        params! {
            "spelling" => body.spelling,
            "meaning" => body.meaning,
            "part_of_speech" => body.part_of_speech.clone()
        },
    ) {
        Ok(_) => {
            let json_response = serde_json::json!({
               "message": "Vocabulary registered successfully"
            });

            Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "message": format!("Failed to update vocabulary: {}", e),
            });

            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

pub async fn delete_vocabulary(
    State(data): State<Arc<AppState>>,
    Json(body): Json<DeleteVocabularyInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    match conn.exec_drop(
        r"DELETE FROM vocabulary WHERE id=(:id);",
        params! {
            "id" => body.id,
        },
    ) {
        Ok(_) => {
            let json_response = serde_json::json!({
               "message": "Vocabulary deleted successfully"
            });

            Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "message": format!("Failed to update vocabulary: {}", e),
            });

            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

pub async fn update_vocabulary(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateVocabularyInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = data.db.get_conn().unwrap();

    let part_of_speech = match PartOfSpeech::from_string(body.part_of_speech.clone()) {
        Ok(value) => value,
        Err(_) => {
            let error_response = serde_json::json!({
                "message": format!("Invalid part_of_speech: {}", body.part_of_speech),
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    match conn.exec_drop(
        r"UPDATE vocabulary SET spelling=(:spelling), meaning=(:meaning), part_of_speech=(:part_of_speech), updated_at=CURTIME() WHERE id=(:id);",
        params! {
            "spelling" => body.spelling,
            "meaning" => body.meaning,
            "part_of_speech" => part_of_speech.text(),
            "id" => body.id,
        },
    ) {
        Ok(_) => {
            let json_response = serde_json::json!({
               "message": "Vocabulary updated successfully"
            });

           Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "message": format!("Failed to update vocabulary: {}", e),
            });
            
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}
