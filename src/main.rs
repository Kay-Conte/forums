mod models;
mod storage;

use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};

use foxhole::{
    framework::run_with_cache,
    sys,
    resolve::{Endpoint, Get, Query, UrlCollect, UrlPart, self},
    action::Html,
    type_cache::TypeCache,
    Route, Response, IntoResponse, action::{Css, Js},
};

use models::{Json, Post, CreatePost, Body, PostId};
use storage::{Database, Counter};

#[cfg(debug_assertions)]
fn root_path() -> PathBuf {
    let mut current = std::env::current_exe().unwrap();

    for _ in 0..3 {
        if !current.pop() {
            panic!("Invalid filestructure")
        }
    }

    current.join("web")
}

#[cfg(not(debug_assertions))]
fn root_path() -> PathBuf {
    let mut current = std::env::current_exe().unwrap();

    if !current.pop() {
        panic!("Invalid filestructure")
    }

    current.join("web")
}

fn post(_get: Get, _post_id: UrlPart, _e: Endpoint) -> Option<Html> {
    let path = root_path().join("post.html");

    let mut file = File::open(path).ok()?;

    let mut s = String::new();

    file.read_to_string(&mut s).ok()?;

    Some(Html(s))
}

fn create_post_page(_get: Get, _post_id: UrlPart, _e: Endpoint) -> Option<Html> {
    let path = root_path().join("create_post.html");

    let mut file = File::open(path).ok()?;

    let mut s = String::new();

    file.read_to_string(&mut s).ok()?;

    Some(Html(s))
}

fn get_posts(_g: Get, UrlPart(root): UrlPart, _e: Endpoint, Query(db): Query<Database>) -> Option<Json<Vec<Post>>> {

    let db = db.lock().ok()?;

    let mut stmt = db.prepare("SELECT id, parent, title, content FROM posts WHERE id = ?1 OR parent = ?1 ORDER BY id").ok()?;

    let root = root.parse::<usize>().ok()?;
    let mut query = stmt.query([root]).ok()?;

    let mut posts = Vec::new();

    while let Some(post) = query.next().ok()? {
        posts.push(Post {
            id: PostId(post.get(0).ok()?),
            parent: post.get(1).ok()?,
            title: post.get(2).ok()?,
            content: post.get(3).ok()?,
        });
    }

    Some(Json(posts))
}

fn create_post(_p: resolve::Post, UrlPart(root): UrlPart, Body(body): Body<CreatePost>, _e: Endpoint, Query(db): Query<Database>, Query(counter): Query<Counter>) -> u16 {
    let Ok(db) = db.lock() else {
        return 500;
    };

    if body.content.contains(['<', '>']) || body.title.contains(['<', '>']) {
        return 400;
    }

    let new_id = counter.write().unwrap().next();

    if db.execute("INSERT INTO posts (id, parent, title, content) VALUES (?1, ?2, ?3, ?4)", (new_id, root, body.title, body.content)).is_err() {
        return 500;
    }

    200
}

fn web(_get: Get, UrlCollect(all): UrlCollect) -> Option<Response<Vec<u8>>> {
    let mut path = root_path();
    let mut last = String::new();

    for part in all {
        path = path.join(part.clone());

        last = part;
    }

    let mut file = File::open(path).ok()?;

    let mut s = String::new();

    file.read_to_string(&mut s).ok()?;

    let res = if last.ends_with(".css") {
        Css(s).response()
    } else if last.ends_with(".js") {
        Js(s).response()
    } else {
        Html(s).response()
    };

    Some(res)
}

fn root(_get: Get, _e: Endpoint) -> Option<Html> {
    let mut file = File::open(root_path().join("index.html")).ok()?;

    let mut s = String::new();

    file.read_to_string(&mut s).ok()?;

    Some(Html(s))
}

fn main() {
    let router = Route::new(sys![root])
        .route("post", sys![post])
        .route("createPost", sys![create_post_page])
        .route("web", sys![web]).route(
        "api",
        Route::empty()
            .route("getPosts", sys![get_posts])
            .route("createPost", sys![create_post])
    );

    let mut cache = TypeCache::new();

    cache.insert::<Database>(Database::new(root_path().join("db")));
    cache.insert::<Counter>(Counter::new());

    run_with_cache("0.0.0.0:8080", router, cache);
}
