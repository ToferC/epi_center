use actix_web::{web, guard};
use actix_files::Files;

use crate::handlers::{
    index,
    api_base,
    org_chart,
    playground_handler,
    graphql,
    graphql_ws,
};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(api_base);
    config.service(org_chart);
    config.service(Files::new("/static", std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static")));
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
