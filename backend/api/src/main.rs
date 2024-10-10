use ::serde::{Deserialize, Serialize};
use axum::routing::post;
use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use model::user::Model as User;
use sea_orm::*;
use sea_orm::{Database, DatabaseConnection};
use uuid::{serde, Uuid};

mod errors;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.expect("Migration failed");

    let app_state = AppState { conn };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/posts", post(create_post))
        .with_state(app_state);

    Ok(router.into())
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    display_name: String,
    username: String,
    email: String,
    avatar_url: String,
}

#[derive(Deserialize, Serialize)]
struct UserDetail {
    id: Uuid,
    display_name: String,
    username: String,
    email: String,
    avatar_url: String,
    user_type: String,
}

#[derive(Deserialize, Serialize)]
struct CreatePost {
    title: String,
    description: String,
    author_id: Uuid,
}

impl From<User> for UserDetail {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            user_type: match user.user_type {
                model::sea_orm_active_enums::UserType::Regular => "Regular".to_string(),
                model::sea_orm_active_enums::UserType::Administrator => "Admin".to_string(),
                model::sea_orm_active_enums::UserType::Moderator => "Moderator".to_string(),
            },
        }
    }
}

async fn create_user(state: State<AppState>, Json(payload): Json<CreateUser>) -> Json<UserDetail> {
    Json(
        model::user::ActiveModel {
            id: Set(Uuid::new_v4()),
            display_name: Set(payload.display_name),
            username: Set(payload.username),
            email: Set(payload.email),
            avatar_url: Set(payload.avatar_url),
            user_type: Set(model::sea_orm_active_enums::UserType::Regular),
            ..Default::default()
        }
        .insert(&state.conn)
        .await
        .unwrap()
        .into(),
    )
}

async fn get_users(state: State<AppState>) -> Json<Vec<UserDetail>> {
    Json(
        model::user::Entity::find()
            .all(&state.conn)
            .await
            .unwrap()
            .into_iter()
            .map(UserDetail::from)
            .collect(),
    )
}

async fn create_post(state: State<AppState>, Json(payload): Json<CreatePost>) -> Json<()> {
    model::post::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(payload.title),
        description: Set(payload.description),
        author_id: Set(payload.author_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&state.conn)
    .await
    .unwrap();

    Json(())
}