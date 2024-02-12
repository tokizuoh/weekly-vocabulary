// use crate::vocabulary::{PartOfSpeech, Vocabulary};
// use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use mysql::{prelude::Queryable, *};
// use serde::Deserialize;
// use std::sync::Arc;

// #[derive(Debug, Deserialize)]
// pub struct DeleteVocabularyInput {
//     pub id: String,
// }

// // TODO: rewrite
// pub async fn delete_vocabulary(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<DeleteVocabularyInput>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let mut conn = data.db.get_conn().unwrap();

//     match conn.exec_drop(
//         r"DELETE FROM vocabulary WHERE id=(:id);",
//         params! {
//             "id" => body.id,
//         },
//     ) {
//         Ok(_) => {
//             let json_response = serde_json::json!({
//                "message": "Vocabulary deleted successfully"
//             });

//             Ok(Json(json_response))
//         }
//         Err(e) => {
//             let error_response = serde_json::json!({
//                 "message": format!("Failed to update vocabulary: {}", e),
//             });

//             Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
//         }
//     }
// }
