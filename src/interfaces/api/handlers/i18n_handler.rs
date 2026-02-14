use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::application::dtos::i18n_dto::{
    LocaleDto, TranslationErrorDto, TranslationRequestDto, TranslationResponseDto,
};
use crate::application::services::i18n_application_service::I18nApplicationService;
use crate::domain::services::i18n_service::{I18nError, Locale};

type AppState = Arc<I18nApplicationService>;

/// Handler for i18n-related API endpoints
pub struct I18nHandler;

impl I18nHandler {
    /// Gets a list of available locales
    pub async fn get_locales(State(service): State<AppState>) -> impl IntoResponse {
        let locales = service.available_locales().await;
        let locale_dtos: Vec<LocaleDto> = locales.into_iter().map(LocaleDto::from).collect();

        (StatusCode::OK, Json(locale_dtos)).into_response()
    }

    /// Translates a key to the requested locale
    pub async fn translate(
        State(service): State<AppState>,
        Query(query): Query<TranslationRequestDto>,
    ) -> impl IntoResponse {
        let locale = match &query.locale {
            Some(locale_str) => match Locale::from_str(locale_str) {
                Some(locale) => Some(locale),
                None => {
                    let error = TranslationErrorDto {
                        key: query.key.clone(),
                        locale: locale_str.clone(),
                        error: format!("Unsupported locale: {}", locale_str),
                    };
                    return (StatusCode::BAD_REQUEST, Json(error)).into_response();
                }
            },
            None => None,
        };

        match service.translate(&query.key, locale).await {
            Ok(text) => {
                let response = TranslationResponseDto {
                    key: query.key,
                    locale: locale.unwrap_or(Locale::default()).as_str().to_string(),
                    text,
                };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(err) => {
                let status = match &err {
                    I18nError::KeyNotFound(_) => StatusCode::NOT_FOUND,
                    I18nError::InvalidLocale(_) => StatusCode::BAD_REQUEST,
                    I18nError::LoadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let error = TranslationErrorDto {
                    key: query.key,
                    locale: locale.unwrap_or(Locale::default()).as_str().to_string(),
                    error: err.to_string(),
                };

                (status, Json(error)).into_response()
            }
        }
    }

    /// Gets all translations for a locale (Axum-compatible: extracts locale from path)
    pub async fn get_translations_by_locale(
        State(service): State<AppState>,
        Path(locale_code): Path<String>,
    ) -> impl IntoResponse {
        Self::get_translations(State(service), locale_code).await
    }

    /// Gets all translations for a locale
    pub async fn get_translations(
        State(_service): State<AppState>,
        locale_code: String,
    ) -> impl IntoResponse {
        let locale = match Locale::from_str(&locale_code) {
            Some(locale) => locale,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": format!("Unsupported locale: {}", locale_code)
                    })),
                )
                    .into_response();
            }
        };

        // This implementation is a bit weird, as we don't have a way to get all translations
        // We should improve the I18nService to support this
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "locale": locale.as_str()
            })),
        )
            .into_response()
    }
}
