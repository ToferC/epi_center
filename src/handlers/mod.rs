mod base;
mod routes;
mod endpoints;

pub use self::routes::configure_services;

pub use self::base::{index, api_base, org_chart};
pub use self::endpoints::*;