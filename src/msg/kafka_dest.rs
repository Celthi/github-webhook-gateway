use std::time::Duration;
use tracing::error;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};


pub async fn produce(brokers: &str, topic_name: &str, payload: &Vec<u8>, key: u64) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    match producer
        .send(
            FutureRecord::to(topic_name)
                .payload(payload)
                .key(&format!("Key {}", key)),
            Duration::from_secs(0),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!(
                "Failed to send message to topic:{topic_name} with error: {:?}.",
                e
            );
        }
    }
}
