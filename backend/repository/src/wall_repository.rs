use std::sync::Arc;

use models::{
    domain::{
        post::Post, post_comment::PostComment, post_like::PostLike, user::User, wall::Wall, Id,
    },
    schema,
};
use sea_orm::{
    DbBackend, DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter,
    Statement,
};

#[derive(Debug, Clone)]
pub struct DbWallRepository {
    db: Arc<DbConn>,
}

impl DbWallRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub type WallPostTuple = (Post, User, Vec<(PostComment, User)>, Vec<(PostLike, User)>);

pub trait WallRepository {
    async fn get_by_id(&self, id: Id<Wall>) -> Result<Option<Wall>, DbErr>;
    async fn create(&self, wall: Wall) -> Result<Id<Wall>, DbErr>;
    async fn get_wall_posts(
        &self,
        wall_id: Id<Wall>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr>;
    async fn get_posts_by_tag(
        &self,
        tag: String,
        user_id: Option<Id<User>>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr>;
    async fn get_feed(
        &self,
        user_id: Option<Id<User>>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr>;
}

impl WallRepository for DbWallRepository {
    async fn get_by_id(&self, id: Id<Wall>) -> Result<Option<Wall>, DbErr> {
        let wall = models::schema::wall::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(wall.map(Wall::from))
    }

    async fn create(&self, Wall: Wall) -> Result<Id<Wall>, DbErr> {
        let wall_model: models::schema::wall::Model = Wall.into();
        let active_model: models::schema::wall::ActiveModel = wall_model.into();

        let inserted = models::schema::wall::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn get_wall_posts(
        &self,
        wall_id: Id<Wall>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr> {
        let wall_posts = models::schema::wall_post::Entity::find()
            .filter(
                models::schema::wall_post::Column::WallId
                    .into_simple_expr()
                    .eq(wall_id.id),
            )
            .all(self.db.as_ref())
            .await?;

        let future_posts = wall_posts.iter().map(|wall_post| {
            let db_ref = self.db.clone();

            async move {
                let Some((post, Some(author))) =
                    models::schema::post::Entity::find_by_id(wall_post.post_id)
                        .find_also_related(schema::user::Entity)
                        .one(db_ref.as_ref())
                        .await?
                else {
                    unreachable!("Post without author");
                };

                let comments = models::schema::post_comment::Entity::find()
                    .filter(
                        models::schema::post_comment::Column::PostId
                            .into_simple_expr()
                            .eq(wall_post.post_id),
                    )
                    .find_also_related(schema::user::Entity)
                    .all(db_ref.as_ref())
                    .await?
                    .into_iter()
                    .map(|(comment, user)| {
                        (comment.into(), user.expect("Comment without user").into())
                    })
                    .collect::<Vec<(PostComment, User)>>();

                let likes = models::schema::post_like::Entity::find()
                    .filter(
                        models::schema::post_like::Column::PostId
                            .into_simple_expr()
                            .eq(wall_post.post_id),
                    )
                    .find_also_related(schema::user::Entity)
                    .all(db_ref.as_ref())
                    .await?
                    .into_iter()
                    .map(|(like, user)| (like.into(), user.expect("Like without user").into()))
                    .collect::<Vec<(PostLike, User)>>();

                let res: WallPostTuple = (Post::from(post), User::from(author), comments, likes);

                Ok(res)
            }
        });

        let posts_awaited: Vec<Result<WallPostTuple, DbErr>> =
            futures::future::join_all(future_posts).await;

        let posts: Vec<WallPostTuple> = posts_awaited.into_iter().collect::<Result<_, _>>()?;

        Ok(posts)
    }

    async fn get_posts_by_tag(
        &self,
        tag: String,
        user_id: Option<Id<User>>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr> {
        let posts = get_posts_by_tag_helper(user_id, &tag, self.db.clone(), offset, limit).await?;

        Ok(posts)
    }

    async fn get_feed(
        &self,
        user_id: Option<Id<User>>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<WallPostTuple>, DbErr> {
        let posts = get_feed_helper(user_id, self.db.clone(), offset, limit).await?;

        Ok(posts)
    }
}

async fn get_posts_by_tag_helper(
    user_id: Option<Id<User>>,
    tag: &str,
    db_conn: Arc<DbConn>,
    offset: i64,
    limit: i64,
) -> Result<Vec<WallPostTuple>, DbErr> {
    let post_tags = if let Some(user_id) = user_id {
        models::schema::post_tag::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
WITH visible_posts AS (
    SELECT * 
    FROM post_tag pt 
    JOIN post p ON p.id = pt.post_id 
    WHERE 
        (pt.tag = $1) AND (p.visibility = 'public'
        OR (p.visibility = 'private' AND EXISTS (
            SELECT 1 
            FROM post_user_visibility puv 
            WHERE puv.post_id = p.id AND puv.user_id = $2 
        ))
        OR (p.visibility = 'private' AND EXISTS (
            SELECT 1 
            FROM post_group_visibility pgv
            JOIN group_member gm ON pgv.group_id = gm.group_id
            WHERE pgv.post_id = p.id AND gm.user_id = $2
        ))
        OR (p.visibility = 'private' AND p.author_id = $2))
),
paged_posts AS (
    SELECT * 
    FROM visible_posts
    ORDER BY created_at DESC
    LIMIT $3 OFFSET $4
)

SELECT * 
FROM paged_posts;
"#,
                [tag.into(), user_id.id.into(), limit.into(), offset.into()],
            ))
            .all(db_conn.as_ref())
            .await?
    } else {
        models::schema::post_tag::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
SELECT * 
from post_tag pt 
join post p on p.id = pt.post_id 
WHERE pt.tag = $1 and p.visibility = 'public'
ORDER BY created_at DESC
LIMIT $2 OFFSET $3;
                "#,
                [tag.into(), limit.into(), offset.into()],
            ))
            .all(db_conn.as_ref())
            .await?
    };

    let future_posts = post_tags.iter().map(|post_tag| {
        let db_ref = db_conn.clone();

        async move {
            let Some((post, Some(author))) =
                models::schema::post::Entity::find_by_id(post_tag.post_id)
                    .find_also_related(schema::user::Entity)
                    .one(db_ref.as_ref())
                    .await?
            else {
                unreachable!("Post without author");
            };

            let comments = models::schema::post_comment::Entity::find()
                .filter(
                    models::schema::post_comment::Column::PostId
                        .into_simple_expr()
                        .eq(post_tag.post_id),
                )
                .find_also_related(schema::user::Entity)
                .all(db_ref.as_ref())
                .await?
                .into_iter()
                .map(|(comment, user)| (comment.into(), user.expect("Comment without user").into()))
                .collect::<Vec<(PostComment, User)>>();

            let likes = models::schema::post_like::Entity::find()
                .filter(
                    models::schema::post_like::Column::PostId
                        .into_simple_expr()
                        .eq(post_tag.post_id),
                )
                .find_also_related(schema::user::Entity)
                .all(db_ref.as_ref())
                .await?
                .into_iter()
                .map(|(like, user)| (like.into(), user.expect("Like without user").into()))
                .collect::<Vec<(PostLike, User)>>();

            let res: WallPostTuple = (Post::from(post), User::from(author), comments, likes);

            Ok(Some(res))
        }
    });

    let posts_awaited: Vec<Result<Option<WallPostTuple>, DbErr>> =
        futures::future::join_all(future_posts).await;

    let posts: Vec<Option<WallPostTuple>> = posts_awaited.into_iter().collect::<Result<_, _>>()?;

    Ok(posts.into_iter().filter_map(|x| x).collect())
}

async fn get_feed_helper(
    user_id: Option<Id<User>>,
    db_conn: Arc<DbConn>,
    offset: i64,
    limit: i64,
) -> Result<Vec<WallPostTuple>, DbErr> {
    let posts = if let Some(user_id) = user_id {
        models::schema::post::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
WITH visible_posts AS (
    SELECT * 
    FROM post p
    WHERE 
        (p.visibility = 'public'
        OR (p.visibility = 'private' AND EXISTS (
            SELECT 1 
            FROM post_user_visibility puv 
            WHERE puv.post_id = p.id AND puv.user_id = $1
        ))
        OR (p.visibility = 'private' AND EXISTS (
            SELECT 1 
            FROM post_group_visibility pgv
            JOIN group_member gm ON pgv.group_id = gm.group_id
            WHERE pgv.post_id = p.id AND gm.user_id = $1
        ))
        OR (p.visibility = 'private' AND p.author_id = $1))
),
paged_posts AS (
    SELECT * 
    FROM visible_posts
    ORDER BY created_at DESC  -- Order posts by the latest first
    LIMIT $2 OFFSET $3
)

SELECT * 
FROM paged_posts;
"#,
                [user_id.id.into(), limit.into(), offset.into()],
            ))
            .all(db_conn.as_ref())
            .await?
    } else {
        models::schema::post::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
SELECT * 
FROM post p
WHERE p.visibility = 'public'
ORDER BY created_at DESC
LIMIT $1 OFFSET $2;
"#,
                [limit.into(), offset.into()],
            ))
            .all(db_conn.as_ref())
            .await?
    };

    let future_posts = posts.into_iter().map(|post| {
        let db_ref = db_conn.clone();

        async move {
            let author = models::schema::user::Entity::find_by_id(post.author_id)
                .one(db_ref.as_ref())
                .await?
                .expect("Post without author");

            let comments = models::schema::post_comment::Entity::find()
                .filter(
                    models::schema::post_comment::Column::PostId
                        .into_simple_expr()
                        .eq(post.id),
                )
                .find_also_related(schema::user::Entity)
                .all(db_ref.as_ref())
                .await?
                .into_iter()
                .map(|(comment, user)| (comment.into(), user.expect("Comment without user").into()))
                .collect::<Vec<(PostComment, User)>>();

            let likes = models::schema::post_like::Entity::find()
                .filter(
                    models::schema::post_like::Column::PostId
                        .into_simple_expr()
                        .eq(post.id),
                )
                .find_also_related(schema::user::Entity)
                .all(db_ref.as_ref())
                .await?
                .into_iter()
                .map(|(like, user)| (like.into(), user.expect("Like without user").into()))
                .collect::<Vec<(PostLike, User)>>();

            let res: WallPostTuple = (Post::from(post), User::from(author), comments, likes);

            Ok(Some(res))
        }
    });

    let posts_awaited: Vec<Result<Option<WallPostTuple>, DbErr>> =
        futures::future::join_all(future_posts).await;

    let posts: Vec<Option<WallPostTuple>> = posts_awaited.into_iter().collect::<Result<_, _>>()?;

    Ok(posts.into_iter().filter_map(|x| x).collect())
}
