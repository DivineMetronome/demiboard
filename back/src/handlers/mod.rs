mod error;
mod types;

use crate::db::model::{Board, ImageNew, Post, PostNew, Thread, ThreadNew, ThreadWithPosts};
use crate::util::multipart;
use crate::util::{
    sse_thread::{Broadcaster, Event},
    GetIdentity,
};
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use error::RequestError;
use serde_json::{json, Value};
use tokio::sync::Mutex;
use types::*;

type Result<T> = std::result::Result<T, RequestError>;

#[get("/boards")]
pub async fn boards(pool: Data<sqlx::PgPool>) -> Result<Json<Vec<Board>>> {
    let boards = Board::fetch_all(pool.as_ref()).await?;
    Ok(Json(boards))
}

#[get("/boards/{board}/catalog")]
pub async fn catalog(pool: Data<sqlx::PgPool>, path: Path<String>) -> Result<Json<Vec<Thread>>> {
    let threads = Thread::fetch_catalog(pool.as_ref(), &path.into_inner()).await?;
    Ok(Json(threads))
}

#[post("/boards/{board}")]
pub async fn new_thread(
    pool: Data<sqlx::PgPool>,
    path: Path<String>,
    identity: Identity,
    mp: Multipart,
) -> Result<Json<Value>> {
    let identity = identity.get();
    let (info, mut images) = multipart::to_payload::<NewThread>(mp).await?;

    if images.len() == 0 {
        return Err(RequestError::BadRequest(
            "Threads should have an image".into(),
        ));
    }

    let new_thread = ThreadNew {
        board: path.into_inner(),
        title: info.title.unwrap_or_default().chars().take(100).collect(),
        name: info.name.unwrap_or_default().chars().take(50).collect(),
        message: info.message.chars().take(5000).collect(),
        image: if images.len() > 0 {
            let i = images.remove(0);
            Some(ImageNew {
                name: i.name,
                path: i.path.clone(),
                preview_path: i.path,
            })
        } else {
            None
        },
    };

    let thread = Thread::post(pool.as_ref(), new_thread, identity).await?;

    Ok(Json(json!({
        "success": true,
        "thread": thread
    })))
}

#[post("/thread/{thread}")]
pub async fn new_post(
    pool: Data<sqlx::PgPool>,
    brd: Data<Mutex<Broadcaster>>,
    path: Path<i32>,
    identity: Identity,
    mp: Multipart,
) -> Result<Json<Value>> {
    let identity = identity.get();
    let (info, mut images) = multipart::to_payload::<NewPost>(mp).await?;

    let new_post = PostNew {
        identity: identity,
        name: info.name.unwrap_or_default().chars().take(50).collect(),
        message: info.message.chars().take(5000).collect(),
        thread: path.into_inner(),
        image: if images.len() > 0 {
            let i = images.remove(0);
            Some(ImageNew {
                name: i.name,
                path: i.path.clone(),
                preview_path: i.path,
            })
        } else {
            None
        },
    };

    let post = Post::post(pool.as_ref(), &new_post).await?;

    brd.lock().await.send(post.thread, Event::Post(&post));

    Ok(Json(json!({
        "success": true,
        "post": post
    })))
}

#[get("/sse/thread/{thread}")]
async fn thread_subscribe(
    brd: Data<Mutex<Broadcaster>>,
    path: Path<i32>,
    pool: Data<sqlx::PgPool>,
) -> Result<HttpResponse> {
    let thread = ThreadWithPosts::fetch(pool.as_ref(), path.into_inner()).await?;

    let rx = brd
        .lock()
        .await
        .new_subscriber(thread)
        .ok_or(RequestError::Teapot)?;

    let mut res = HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .no_chunking(0)
        .streaming(rx);

    // no_chunking forces a Content-Length header, which breaks SSE
    // was introduced in a recent alpha update
    // TODO: find a better way
    res.headers_mut()
        .remove(actix_web::http::header::CONTENT_LENGTH);

    Ok(res)
}
