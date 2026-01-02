use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::usecase::user_service::UserService;
use crate::usecase::ports::UserRepository;
use super::dto::{CreateUserReq, UpdateUserReq, ListUsersQuery};

#[derive(Clone)]
pub struct HttpState<R: UserRepository> {
    pub user_service: Arc<UserService<R>>,
}

pub async fn create_user<R: UserRepository>(
    State(st): State<HttpState<R>>,
    Json(req): Json<CreateUserReq>,
) -> impl IntoResponse {
    match st.user_service.create_user(req.username, req.email, req.active).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_user<R: UserRepository>(
    State(st): State<HttpState<R>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match st.user_service.get_user(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn list_users<R: UserRepository>(
    State(st): State<HttpState<R>>,
    Query(q): Query<ListUsersQuery>,
) -> impl IntoResponse {
    match st.user_service.list_users(q.active, q.limit, q.offset).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn update_user<R: UserRepository>(
    State(st): State<HttpState<R>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserReq>,
) -> impl IntoResponse {
    match st.user_service.update_user(id, req.username, req.email, req.active).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn delete_user<R: UserRepository>(
    State(st): State<HttpState<R>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match st.user_service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

fn map_error(err: DomainError) -> (StatusCode, String) {
    match err {
        DomainError::NotFound => (StatusCode::NOT_FOUND, "user not found".into()),
        DomainError::Conflict(s) => (StatusCode::CONFLICT, s),
        DomainError::Validation(s) => (StatusCode::BAD_REQUEST, s),
        DomainError::Unexpected(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
    }
}
