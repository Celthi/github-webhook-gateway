use std::thread;
use tracing::level_filters;
use webhook_gateway::config_env;
use webhook_gateway::events;
use webhook_gateway::events::msg::queue;
use webhook_gateway::web;

fn main() {
    let filter = level_filters::LevelFilter::INFO;
    tracing_subscriber::fmt().with_max_level(filter).init();
    config_env::ensure_config();
    queue::init_channels();
    let mut v = vec![];
    let j = thread::spawn(|| {
        web::event_loop().expect("web event loop failing."); // fine to crash as cannot start the web server
    });
    v.push(j);
    let j = thread::spawn(|| {
        events::msg::consumer::event_loop();
    });
    v.push(j);
    for t in v {
        t.join().expect("cannot join the thread.");
    }
}
