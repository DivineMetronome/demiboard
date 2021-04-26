use futures::join;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Result};
use time::OffsetDateTime;

#[derive(Serialize)]
pub struct Board {
    code: String,
    name: String,
    description: String,
}
impl Board {
    pub async fn fetch_all(pool: &PgPool) -> Result<Vec<Self>> {
        sqlx::query_as!(Board, "SELECT * FROM boards")
            .fetch_all(pool)
            .await
    }
    pub async fn update_locks(pool: &PgPool, board: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE threads \
            SET open = false \
            WHERE id IN ( \
                SELECT id FROM threads \
                WHERE board = $1 \
                ORDER BY last_updated DESC \
                OFFSET 100 \
                LIMIT 1 \
            )",
            board
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

pub struct ThreadNew {
    pub board: String,
    pub title: String,
    pub name: String,
    pub message: String,
    pub image: Option<ImageNew>,
}

#[derive(Serialize)]
pub struct Thread {
    id: i32,
    last_updated: OffsetDateTime,
    open: bool,
    board: String,
    title: String,
}
impl Thread {
    pub async fn fetch(pool: &PgPool, thread_id: i32) -> Result<Option<Self>> {
        sqlx::query_as!(Thread, "SELECT * FROM threads WHERE id = $1", thread_id)
            .fetch_optional(pool)
            .await
    }
    pub async fn fetch_catalog(pool: &PgPool, board: &str) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Thread,
            "SELECT * FROM threads \
            WHERE board = $1 AND open = true \
            ORDER BY last_updated DESC \
            LIMIT 100",
            board
        )
        .fetch_all(pool)
        .await
    }
    pub async fn post(
        pool: &PgPool,
        new_thread: ThreadNew,
        identity: String,
    ) -> Result<ThreadWithPosts> {
        let thread: Thread = sqlx::query_as!(
            Thread,
            "INSERT INTO threads (board, title) \
            VALUES ($1, $2) \
            RETURNING id, last_updated, open, board, title",
            new_thread.board,
            new_thread.title
        )
        .fetch_one(pool)
        .await?;

        let post = Post::post(
            pool,
            &PostNew {
                thread: thread.id,
                name: new_thread.name,
                message: new_thread.message,
                identity: identity,
                image: new_thread.image,
            },
        )
        .await?;

        Board::update_locks(pool, &thread.board).await?;

        Ok((thread, vec![post]).into())
    }
}

#[derive(Serialize)]
pub struct ThreadWithPosts {
    pub id: i32,
    board: String,
    pub open: bool,
    title: String,
    posts: Vec<Post>,
}

impl From<(Thread, Vec<Post>)> for ThreadWithPosts {
    fn from((thread, posts): (Thread, Vec<Post>)) -> Self {
        ThreadWithPosts {
            id: thread.id,
            board: thread.board,
            open: thread.open,
            title: thread.title,
            posts,
        }
    }
}

impl ThreadWithPosts {
    pub async fn fetch(pool: &PgPool, thread_id: i32) -> Result<Option<Self>> {
        if let Some(thread) = Thread::fetch(pool, thread_id).await? {
            let posts = Post::fetch_for_thread(pool, thread_id).await?;
            Ok(Some((thread, posts).into()))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize)]
pub struct Post {
    id: i64,
    pub thread: i32,
    name: String,
    timestamp: i64,
    message: String,
    image: Option<Image>,
}

struct PostInner {
    id: i64,
    thread: i32,
    name: String,
    date: OffsetDateTime,
    message: String,
    identity: String,
    image_id: Option<i64>,
    image_name: Option<String>,
    image_path: Option<String>,
    image_preview_path: Option<String>,
}
pub struct PostNew {
    pub thread: i32,
    pub name: String,
    pub message: String,
    pub identity: String,
    pub image: Option<ImageNew>,
}
impl From<PostInner> for Post {
    fn from(pi: PostInner) -> Self {
        Post {
            id: pi.id,
            thread: pi.thread,
            name: pi.name,
            timestamp: pi.date.timestamp(),
            message: pi.message,
            image: if let Some(image_id) = pi.image_id {
                Some(Image {
                    id: image_id,
                    name: pi.image_name.unwrap(),
                    path: pi.image_path.unwrap(),
                    preview_path: pi.image_preview_path.unwrap(),
                })
            } else {
                None
            },
        }
    }
}

impl Post {
    pub async fn fetch_for_thread(pool: &PgPool, thread_id: i32) -> Result<Vec<Self>> {
        let res = sqlx::query_as!(PostInner, "\
          SELECT p.id, p.message, p.date, p.name, p.thread, p.identity, \
            i.id as image_id, i.name as image_name,i.path as image_path,i.preview_path as image_preview_path \
          FROM posts p \
          LEFT JOIN images i ON p.image = i.id \
          WHERE p.thread = $1 \
          ORDER BY id ASC", thread_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|pi| pi.into())
        .collect();

        Ok(res)
    }

    pub async fn post(pool: &PgPool, post: &PostNew) -> Result<Self> {
        let image = if let Some(image) = &post.image {
            Some(Image::post(&pool, image).await?)
        } else {
            None
        };

        // sqlx can't deserialize rows if some optional struct fields aren't present
        // so we have to add them to the query as NULL
        // TODO may be there is or will be a better solution
        let mut post: Post = sqlx::query_as!(
            PostInner,
            "INSERT INTO posts (thread, name, message, identity, image) \
            VALUES ($1, $2, $3, $4, $5) \
            RETURNING id, message, date, name, thread, identity, \
            NULL::bigint as image_id, NULL as image_name, NULL as image_path, NULL as image_preview_path",
            post.thread,
            post.name,
            post.message,
            post.identity,
            image.as_ref().map(|image| image.id)
        )
        .fetch_one(pool)
        .await?
        .into();

        post.image = image;

        Ok(post)
    }
}

#[derive(Serialize)]
pub struct Image {
    id: i64,
    name: String,
    path: String,
    preview_path: String,
}

pub struct ImageNew {
    pub name: String,
    pub path: String,
    pub preview_path: String,
}
impl Image {
    pub async fn fetch(pool: &PgPool, id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM images WHERE id = $1", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn post(pool: &PgPool, image: &ImageNew) -> Result<Self> {
        sqlx::query_as!(
            Image,
            "INSERT INTO images  (name, path, preview_path) \
            VALUES ($1, $2, $3) \
            RETURNING id, name, path, preview_path",
            image.name,
            image.path,
            image.preview_path
        )
        .fetch_one(pool)
        .await
    }
}
