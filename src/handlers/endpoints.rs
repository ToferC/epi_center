use actix_web::{web, HttpResponse, HttpRequest, Result};
use async_graphql::http::{GraphiQLSource};
use async_graphql::Schema;

use async_graphql_actix_web::{GraphQLSubscription,
    GraphQLRequest, GraphQLResponse};

use crate::models;
use crate::graphql::{AppSchema};


pub async fn playground_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn graphql(
    schema: web::Data<AppSchema>,
    http_request: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    
    let mut query = req.into_inner();

    let maybe_role_id = models::get_claim(http_request);

    // insert claim data into query or error for response
    match maybe_role_id {
        Ok((role, uuid, exp_time)) => {
            query = query.data(role);
            query = query.data(uuid);
            query = query.data(exp_time)
        },
        Err(e) => {
            query = query.data(e);
        }
    };

    schema.execute(query).await.into()
}

pub async fn graphql_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}
