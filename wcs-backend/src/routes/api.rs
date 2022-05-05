use actix_web::{Result, Responder, web};
use serde::{Serialize, Deserialize};

use crate::util::similarity as sim;
use crate::util::transcode as tcd;

#[derive(Deserialize)]
pub struct SimilarityRequest {
    first: String,
    second: String,
}

#[derive(Serialize)]
pub struct SimilarityResponse {
    string_similarity: f32,
    phonetic_similarity: f32,
}

pub async fn get_similarity(request: web::Query<SimilarityRequest>)
  -> Result<impl Responder> {

    let string_similarity = sim::similarity(
        &request.first, &request.second,
        sim::damerau_levenshtein_dist
    );

    let phonetic_similarity = sim::similarity(
        &tcd::polyphone_transcode(&request.first),
        &tcd::polyphone_transcode(&request.second),
        sim::damerau_levenshtein_dist
    );

    Ok(web::Json(
        SimilarityResponse {
            string_similarity,
            phonetic_similarity
        }
    ))
}