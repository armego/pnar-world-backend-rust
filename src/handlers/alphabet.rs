use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    constants::alphabet::{convert_kbf_to_pnar, convert_pnar_to_kbf, PNAR_ALPHABET},
    error::AppError,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConvertTextRequest {
    #[schema(example = "kiniise")]
    pub text: String,
    #[schema(example = "kbf_to_pnar")]
    pub direction: ConversionDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ConversionDirection {
    #[serde(rename = "kbf_to_pnar")]
    KbfToPnar,
    #[serde(rename = "pnar_to_kbf")]
    PnarToKbf,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConvertTextResponse {
    #[schema(example = "kiniïsæ")]
    pub converted_text: String,
    #[schema(example = "kiniise")]
    pub original_text: String,
    pub direction: ConversionDirection,
}

/// Get all Pnar alphabet characters (Public endpoint)
#[utoipa::path(
    get,
    path = "/api/v1/alphabets",
    tag = "alphabets",
    responses(
        (status = 200, description = "List of alphabet characters", body = [crate::constants::alphabet::PnarCharacter]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_alphabets() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(&PNAR_ALPHABET[..]))
}

/// Convert text between Pnar and keyboard-friendly format (Public endpoint)
#[utoipa::path(
    post,
    path = "/api/v1/alphabets/convert",
    tag = "alphabets",
    request_body = ConvertTextRequest,
    responses(
        (status = 200, description = "Converted text", body = ConvertTextResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn convert_text(
    convert_request: web::Json<ConvertTextRequest>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ConvertTextRequest { text, direction } = convert_request.into_inner();
    
    let converted_text = match direction {
        ConversionDirection::KbfToPnar => convert_kbf_to_pnar(&text),
        ConversionDirection::PnarToKbf => convert_pnar_to_kbf(&text),
    };

    // Log alphabet conversion usage (no database required for this analytics)
    tracing::info!(
        "Alphabet conversion: {} -> {} (direction: {:?}, ip: {:?})",
        text,
        converted_text,
        direction,
        req.peer_addr().map(|addr| addr.ip().to_string())
    );

    let response = ConvertTextResponse {
        converted_text,
        original_text: text,
        direction,
    };

    Ok(HttpResponse::Ok().json(response))
}
