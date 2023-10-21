mod models;
mod storage;

use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use foxhole::{
    framework::run_with_cache,
    sys,
    systems::{Endpoint, Html, Query, UrlCollect, UrlPart, Css, Js, self},
    type_cache::TypeCache,
    Get, Route, Response, IntoResponse,
};

use models::{Json, Post, User, UserId, CreatePost, Body};
use storage::{Auth, Forum, Posts, Users};
use uuid::Uuid;

fn root_path() -> PathBuf {
    let mut current = std::env::current_exe().unwrap();

    for _ in 0..3 {
        if !current.pop() {
            panic!("Invalid filestructure")
        }
    }

    current.join("web")
}

fn get_user(_get: Get, UrlPart(user_id): UrlPart, _e: Endpoint, Query(users): Query<Users>) -> Option<Json<User>> {
    let users = &users.read().unwrap().0;

    users.get(&UserId(user_id)).map(|u| Json(u.clone()))
}

fn get_post(_g: Get, UrlPart(post_id): UrlPart, _e: Endpoint, Query(posts): Query<Posts>) -> Option<Json<Post>> {
    unimplemented!("get_post is not yet implemented");

    let posts = &posts.read().unwrap().0;
}

fn get_posts(_g: Get, UrlPart(amount): UrlPart, _e: Endpoint, Query(posts): Query<Posts>) -> Option<Json<Vec<Post>>> {
    let amount = amount.parse::<usize>().ok()?;
    let posts = &posts.read().unwrap().0;

    Some(Json(posts[0..amount.min(posts.len())].to_vec()))
}

fn create_post(_p: systems::Post, Body(body): Body<CreatePost>, _e: Endpoint, Query(posts): Query<Posts>) -> u16 {
    posts.write().unwrap().0.push(Post {
        user_id: UserId("0".to_string()),
        title: body.title,
        content: body.content,
    });

    200
}

fn create_user(_get: Get, UrlPart(user_name): UrlPart, _e: Endpoint, Query(users): Query<Users>) -> Json<UserId> {
    let unique = UserId(Uuid::new_v4().to_string());

    let user = User { name: user_name };

    let users = &mut users.write().unwrap().0;

    users.insert(unique.clone(), user);

    Json(unique)
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
    let router = Route::new(sys![root]).route("web", sys![web]).route(
        "api",
        Route::empty()
            .route("getPost", sys![get_post])
            .route("getPosts", sys![get_posts])
            .route("getUser", sys![get_user])
            .route("createPost", sys![create_post])
            .route("createUser", sys![create_user]),
    );

    let mut cache = TypeCache::new();

    cache.insert::<Forum>(Arc::new(RwLock::new(Forum(HashMap::new()))));
    cache.insert::<Users>(Arc::new(RwLock::new(Users(HashMap::new()))));
    cache.insert::<Auth>(Arc::new(RwLock::new(Auth(HashMap::new()))));
    cache.insert::<Posts>(Arc::new(RwLock::new(Posts(Vec::new()))));

    run_with_cache("0.0.0.0:8080", router, cache);
}
