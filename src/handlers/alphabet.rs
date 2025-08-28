use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::{
    constants::alphabet::{convert_kbf_to_pnar, convert_pnar_to_kbf, PNAR_ALPHABET},
    error::AppError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertTextRequest {
    pub text: String,
    pub direction: ConversionDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionDirection {
    #[serde(rename = "kbf_to_pnar")]
    KbfToPnar,
    #[serde(rename = "pnar_to_kbf")]
    PnarToKbf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertTextResponse {
    pub converted_text: String,
    pub original_text: String,
    pub direction: ConversionDirection,
}

/// Get all Pnar alphabet characters (Public endpoint)
pub async fn list_alphabets() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(&PNAR_ALPHABET[..]))
}

/// Convert text between Pnar and keyboard-friendly format (Public endpoint)
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
