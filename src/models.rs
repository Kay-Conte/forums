use foxhole::{IntoResponse, Response, resolve::{Resolve, ResolveGuard}};
use serde::{Serialize, Deserialize};

pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T> where T: Serialize {
    fn response(self) -> Response<Vec<u8>> {
        let json = serde_json::to_string(&self.0).unwrap();

        Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("Content-Length", format!("{}", json.len()))
            .body(json.into_bytes())
            .unwrap()
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostId(pub u64);

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
}

pub struct Body<T>(pub T);

#[derive(Deserialize, Debug)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

impl<'a, T> Resolve<'a> for Body<T> where T: Deserialize<'a> + 'static {
    type Output = Self;

    fn resolve(ctx: &'a foxhole::RequestState, _path_iter: &mut foxhole::PathIter) -> ResolveGuard<Self::Output> {
        let Ok(s) = std::str::from_utf8(ctx.request.body().get()) else {
            return ResolveGuard::None;
        };

        let Ok(b) = serde_json::from_str::<T>(&s) else {
            return ResolveGuard::None;
        };

        ResolveGuard::Value(Body(b))
    }
}
