use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use models::errors::AppResult;
use serde::{Deserialize, Serialize};

use usecase::location::create_location::CreateLocationUseCase;
use uuid::Uuid;

use crate::{extractors::json_extractor::Json, AppState};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateLocationResponse {
    id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CreateLocationRequest {
    name: String,
    latitude: f64,
    longitude: f64,
    picture_url: Option<String>,
}

async fn create_location(
    state: State<AppState>,
    Json(payload): Json<CreateLocationRequest>,
) -> AppResult<Json<CreateLocationResponse>> {
    let create_location_use_case = CreateLocationUseCase::new(state.location_repository.clone());

    let input = usecase::location::create_location::CreateLocationInput {
        name: payload.name,
        latitude: payload.latitude,
        longitude: payload.longitude,
        picture_url: payload.picture_url,
    };

    let result = create_location_use_case.execute(input).await?;

    Ok(Json(CreateLocationResponse { id: result.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeleteLocationResponse {
    success: bool,
}

async fn delete_location(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<DeleteLocationResponse>> {
    let delete_location_use_case = usecase::location::delete_location::DeleteLocationUseCase::new(
        state.location_repository.clone(),
    );

    let input = usecase::location::delete_location::DeleteLocationInput { id };

    let result = delete_location_use_case.execute(input).await?;

    Ok(Json(DeleteLocationResponse {
        success: result.success,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SearchLocationRequest {
    query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchLocationResponse {
    locations: Vec<SearchLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchLocation {
    id: Uuid,
    name: String,
    latitude: f64,
    longitude: f64,
    picture_url: Option<String>,
}

async fn search_location(
    state: State<AppState>,
    Query(params): Query<SearchLocationRequest>,
) -> AppResult<Json<SearchLocationResponse>> {
    let search_location_use_case = usecase::location::search_location::SearchLocationUseCase::new(
        state.location_repository.clone(),
    );

    let input = usecase::location::search_location::SearchLocationInput {
        query: params.query,
    };

    let result = search_location_use_case.execute(input).await?;

    Ok(Json(SearchLocationResponse {
        locations: result
            .locations
            .into_iter()
            .map(|loc| SearchLocation {
                id: loc.id.into(),
                name: loc.name,
                latitude: loc.latitude,
                longitude: loc.longitude,
                picture_url: loc.picture_url,
            })
            .collect(),
    }))
}

pub fn location_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_location))
        .route("/:id", delete(delete_location))
        .route("/search", get(search_location))
}
