use std::sync::Mutex;
use std::time::Duration;

use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;

lazy_static! {
    static ref BOOTSTRAP_SERVERS: String = 
        std::env::var("BOOTSTRAP_SERVERS").expect("Can't read Kafka broker address");
    static ref SECURITY_PROTOCOL: String = 
        std::env::var("SECURITY_PROTOCOL").expect("Can't read Kafka security protocol");
    static ref SASL_MECHANISMS: String = 
        std::env::var("SASL_MECHANISMS").expect("Can't read Kafka sasl.mechanisms");
    static ref SASL_USERNAME: String = 
        std::env::var("SASL_USERNAME").expect("Can't read Kafka sasl.username");
    static ref SASL_PASSWORD: String = 
        std::env::var("SASL_PASSWORD").expect("Can't read Kafka sasl.password");
}

pub(crate) fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", BOOTSTRAP_SERVERS.as_str())
        .set("message.timeout.ms", "5000")
        .set("security.protocol", SECURITY_PROTOCOL.as_str())
        .set("sasl.mechanisms", SASL_MECHANISMS.as_str())
        .set("sasl.username", SASL_USERNAME.as_str())
        .set("sasl.password", SASL_PASSWORD.as_str())
        .set("api.version.request", "false")
        .create()
        .expect("Producer creation failed")
}

pub(crate) fn create_consumer(group_id: String, topic: &str) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", "pkc-ymrq7.us-east-2.aws.confluent.cloud:9092")
        .set("security.protocol", SECURITY_PROTOCOL.as_str())
        .set("sasl.mechanisms", SASL_MECHANISMS.as_str())
        .set("sasl.username", SASL_USERNAME.as_str())
        .set("sasl.password", SASL_PASSWORD.as_str())
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&vec![topic])
        .expect("Can't subscribe to specified topics");

    consumer
}

pub(crate) fn get_kafka_consumer_group(kafka_consumer_counter: &Mutex<i32>) -> String {
    let mut counter = kafka_consumer_counter.lock().expect("Can't lock counter");
    *counter += 1;
    format!("graphql-group-{}", *counter)
}

pub(crate) async fn send_message(producer: &FutureProducer, topic: &str, message: String, key: String) {
    let send_to_kafka_result = producer
        .send(
            FutureRecord::to(topic)
                .payload(&message)
                .key(&key),
            Timeout::After(Duration::from_secs(3)),
        )
        .await;

    match send_to_kafka_result {
        Ok(_) => println!("Message was sent to topic: {}", topic),
        Err(res) => println!("Message wasn't sent: {}", res.0),
    }
}