use std::sync::Mutex;

use async_graphql::*;
use futures::{Stream, StreamExt};
use rdkafka::{Message};

// use crate::kafka::create_consumer;
// use crate::kafka::get_kafka_consumer_group;
use crate::models::TravelData;
use crate::models::{Trip, Person};
use crate::common_utils::{is_analyst};
use crate::common_utils::{RoleGuard, Role};

pub struct Subscription;

#[Subscription]
impl Subscription {
    /// Subscription service that returns a stream of the latest trips created
    #[graphql(
        guard = "RoleGuard::new(Role::Analyst)",
        visible = "is_analyst",
    )]
    async fn latest_trip<'ctx>(
        &self, 
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = Trip> + 'ctx {
        let kafka_consumer_counter = ctx 
            .data::<Mutex<i32>>()
            .expect("Can't get Kafka consumer counter");

        let consumer_group_id = get_kafka_consumer_group(kafka_consumer_counter);
        let consumer = create_consumer(consumer_group_id, "trips");

        async_stream::stream! {
            let mut stream = consumer.stream();

            while let Some(value) = stream.next().await {
                yield match value {
                    Ok(message) => {
                        let payload = message.payload().expect("Kafka message shoudl contain payload");
                        let message = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&message).expect("Can't deserialize Trip")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }

    /// Subscription to live person feed
    #[graphql(
        //guard = "RoleGuard::new(Role::Analyst)",
        //visible = "is_analyst",
    )]
    async fn latest_person<'ctx>(
        &self, 
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = Person> + 'ctx {
        let kafka_consumer_counter = ctx 
            .data::<Mutex<i32>>()
            .expect("Can't get Kafka consumer counter");

        let consumer_group_id = get_kafka_consumer_group(kafka_consumer_counter);
        let consumer = create_consumer(consumer_group_id, "people");

        async_stream::stream! {
            let mut stream = consumer.stream();

            while let Some(value) = stream.next().await {
                yield match value {
                    Ok(message) => {
                        let payload = message.payload().expect("Kafka message shoudl contain payload");
                        let message = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&message).expect("Can't deserialize Trip")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }

    /// Subscription to raw ArriveCan feed from PIL
    #[graphql(
        //guard = "RoleGuard::new(Role::Analyst)",
        //visible = "is_analyst",
    )]
    async fn latest_traveller<'ctx>(
        &self, 
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = TravelData> + 'ctx {
        let kafka_consumer_counter = ctx 
            .data::<Mutex<i32>>()
            .expect("Can't get Kafka consumer counter");

        let consumer_group_id = get_kafka_consumer_group(kafka_consumer_counter);
        let consumer = create_consumer(consumer_group_id, "arrivecan_pil");

        async_stream::stream! {
            let mut stream = consumer.stream();

            while let Some(value) = stream.next().await {
                yield match value {
                    Ok(message) => {
                        let payload = message.payload().expect("Kafka message shoudl contain payload");
                        let message = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&message).expect("Can't deserialize ArriveCan PIL message")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }
}