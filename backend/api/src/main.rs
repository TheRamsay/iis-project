use std::sync::Arc;

use ::serde::{Deserialize, Serialize};
use axum::routing::post;
use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use repository::group_repository::DbGroupRepository;
use repository::user_repository::{DbUserRepository, UserRepository};
use repository::wall_repository::DbWallRepository;
use routes::group::group_routes;
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
    pub wall_repository: DbWallRepository,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    println!("Starting server...");
    println!("Starting server...");
    println!("Starting server...");
    println!("Starting server...");
    println!("Starting server...");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.expect("Migration failed");

    let app_state = AppState {
        user_repository: DbUserRepository::new(Arc::new(conn.clone())),
        group_repository: DbGroupRepository::new(Arc::new(conn.clone())),
        wall_repository: DbWallRepository::new(Arc::new(conn.clone())),
        conn: conn.clone(),
    };

    let router = Router::new()
        .nest("/api/users", user_routes())
        .nest("/api/groups", group_routes())
        .with_state(app_state);

    Ok(router.into())
}
