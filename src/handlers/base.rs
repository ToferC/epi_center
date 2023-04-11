use actix_web::{web, get, HttpResponse, HttpRequest, Responder};
//use actix_identity::Identity;

use tera::Context;

use crate::AppData;
use crate::database::PostgresPool;

#[get("/")]
pub async fn index(data: web::Data<AppData>, _req:HttpRequest) -> impl Responder {
    let ctx = Context::new(); 
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    // Explicitly set the content type to text/html
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}

#[get("/org_chart")]
pub async fn org_chart(data: web::Data<AppData>, _req:HttpRequest) -> impl Responder {
    let ctx = Context::new(); 
    let rendered = data.tmpl.render("org_chart.html", &ctx).unwrap();
    // Explicitly set the content type to text/html
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}

#[get("/{lang}/api")]
pub async fn api_base(
    data: web::Data<AppData>,
    _pool: web::Data<PostgresPool>,
    _lang: web::Path<String>,
    _req: HttpRequest,
    // id: Identity,
) -> impl Responder {

    let ctx = Context::new(); 
    let rendered = data.tmpl.render("api_base.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


