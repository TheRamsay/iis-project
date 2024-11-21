use std::sync::Arc;

use ::serde::{Deserialize, Serialize};
use axum::routing::post;
use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use repository::cloudinary_repository::{CloudinaryRepository, GenericRepository};
use repository::group_join_request_repository::DbGroupJoinRequestRepository;
use repository::group_member_repository::DbGroupMemberRepository;
use repository::group_repository::DbGroupRepository;
use repository::post_repository::DbPostRepository;
use repository::user_repository::{DbUserRepository, UserRepository};
use repository::wall_repository::DbWallRepository;
use routes::auth::auth_routes;
use routes::group::group_routes;
use routes::group_join_request::group_join_request_router;
use routes::post::post_routes;
use routes::user::user_routes;
use sea_orm::*;
use sea_orm::{Database, DatabaseConnection};
use uuid::{serde, Uuid};

mod extractors;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub user_repository: DbUserRepository,
    pub group_repository: DbGroupRepository,
    pub post_repository: DbPostRepository,
    pub cloudinary_repository: GenericRepository,
    pub wall_repository: DbWallRepository,
    pub group_member_repository: DbGroupMemberRepository,
    pub group_join_request_repository: DbGroupJoinRequestRepository,
    pub jwt_secret: String,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.expect("Migration failed");

    let app_state = AppState {
        user_repository: DbUserRepository::new(Arc::new(conn.clone())),
        group_repository: DbGroupRepository::new(Arc::new(conn.clone())),
        wall_repository: DbWallRepository::new(Arc::new(conn.clone())),
        group_member_repository: DbGroupMemberRepository::new(Arc::new(conn.clone())),
        group_join_request_repository: DbGroupJoinRequestRepository::new(Arc::new(conn.clone())),
        post_repository: DbPostRepository::new(Arc::new(conn.clone())),
        cloudinary_repository: GenericRepository {},
        conn: conn.clone(),
        jwt_secret,
    };

    let router = Router::new()
        .nest("/api/users", user_routes())
        .nest("/api/groups", group_routes())
        .nest("/api/auth", auth_routes())
        .nest("/api/group-join-requests", group_join_request_router())
        .nest("/api/posts", post_routes())
        .with_state(app_state);

    Ok(router.into())
}
