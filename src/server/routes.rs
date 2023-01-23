use std::convert::Infallible;
use crate::schema;

use async_graphql::{Request, Schema, http::{playground_source, GraphQLPlaygroundConfig}};
use async_graphql_warp::{GraphQLResponse as GQLResponse};
use serde_json::json;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply, http::Response, reply::json};

// Check that the server is alive
async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {

    // Build the GraphQL schema
    let schema = schema::build_schema().finish();

    let health = warp::path::end().and_then(health);

    // Graphql query and subscription handler
    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GQLResponse::from(schema.execute(request).await))
            }
        )
    ));

    // GraphQL
    let graphql_playground = warp::path("playground").map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // Wire together all the routes
    health
        .or(graphql_handler)
        .or(graphql_playground)
        .boxed()
}