use warp::{filters::BoxedFilter, path, Filter};

use crate::models::request::GrantRequest;

// Sets the path prefix for this API.
// host:port//<path prefix>/<specific API route>
fn path_prefix() -> BoxedFilter<()> {
    path!("gnap" / ..).boxed()
}

pub fn grant_post() -> BoxedFilter<(GrantRequest,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post().and(path_prefix()).and(json_body).boxed()
}

pub fn grant_options() -> BoxedFilter<()> {
    warp::options() // 3.
        .and(path_prefix())
        .and(path("grant"))
        .boxed()
}
