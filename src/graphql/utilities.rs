use std::sync::{Arc, Mutex};
use crate::{database::PostgresPool};

use async_graphql::*;
use diesel::{PgConnection};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

use crate::graphql::{Mutation, query::Query}; // Removed Subscription

// use crate::kafka::{create_producer};

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema_with_context(pg_pool: PostgresPool) -> async_graphql::Schema<Query, Mutation, EmptySubscription> {
    
    //let cloned_conn = pg_pool.clone().get().expect("Unable to connect to db");
    
    let arc_pool = Arc::new(pg_pool);

    /*
    let countries = Arc::new(Mutex::new(Country::load_into_hash(&cloned_conn)));
    let places = Arc::new(Mutex::new(Place::load_into_hash(&cloned_conn)));
    let vaccines = Vaccine::load_into_hash(&cloned_conn);
    */
    let identity: Option<String> = None;

    let kafka_consumer_counter = Mutex::new(0);
    
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        // Database connection
        .data(arc_pool)
        // Live cached data -> may want to remove once dataloaders in place
        /*
        .data(countries)
        .data(places)
        .data(vaccines)
         */
        .data(identity)
        // Kafka
        // .data(create_producer())
        .data(kafka_consumer_counter)
        .finish()
}

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_from_context(ctx: &Context<'_>) -> Conn {
    ctx.data::<Arc<PostgresPool>>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}