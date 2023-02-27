use actix_web::{web, guard};

use crate::handlers::{
    index,
    api_base,
    playground_handler,
    graphql,
    graphql_ws,
    
    // API
    // get_trips,
    // get_trip_by_id,
    // edit_trip,
    // delete_trip,
    // add_trip,
};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(api_base);
    // API use
    // Playground
    config.route("/playground", web::post().to(graphql));
    config.route("/playground", web::get().to(playground_handler));
    // Websocket
    config.service(
        web::resource("/graphql")
        .route(
            web::get()
            .guard(guard::Header("upgrade", "websocket"))
            .to(graphql_ws),
        )
        .route(web::post().to(graphql))
    );
}
