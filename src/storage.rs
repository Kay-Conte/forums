use std::{collections::HashMap, sync::{Arc, RwLock}};

use foxhole::type_cache::TypeCacheKey;

use crate::models::{Post, UserId, User};

pub struct Forum(pub HashMap<String, Vec<Post>>);

impl TypeCacheKey for Forum {
    type Value = Arc<RwLock<Forum>>;
}

pub struct Auth(pub HashMap<String, UserId>);

impl TypeCacheKey for Auth {
    type Value = Arc<RwLock<Auth>>;
}

pub struct Users(pub HashMap<UserId, User>);

impl TypeCacheKey for Users {
    type Value = Arc<RwLock<Users>>;
}

pub struct Posts(pub Vec<Post>);

impl TypeCacheKey for Posts {
    type Value = Arc<RwLock<Posts>>;
}


