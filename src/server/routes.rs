use serde_json::json;
use warp::{Filter, Rejection, Reply};
use warp::filters::BoxedFilter;
use warp::reply::json;

// Check that the server is alive
async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let health = warp::path::end().and_then(health);

    health.boxed()
}