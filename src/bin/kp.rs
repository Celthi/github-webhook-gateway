use std::time::Duration;

use tracing::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::{OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use tracing::{level_filters};


async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add(
                            "header_key",
                            "header_value",
                        )),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            info!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {
  
    let filter = level_filters::LevelFilter::INFO;
    tracing_subscriber::fmt().with_max_level(filter).init();
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);


    let topic = "test";
    let brokers = "<ip>:31433";
    produce(brokers, topic).await;
}
