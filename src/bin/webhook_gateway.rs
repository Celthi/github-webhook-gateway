use webhook_gateway::config_env;
use webhook_gateway::message;
use webhook_gateway::queue;
use webhook_gateway::web;
use std::thread;
fn main() {
    config_env::ensure_config();
    queue::init_channels();
    let mut v = vec![];
    let j = thread::spawn(|| {
        web::event_loop().expect("web event loop failing."); // fine to crash as cannot start the web server
    });
    v.push(j);
    let j = thread::spawn(|| {
        message::consumer::event_loop();
    });
    v.push(j);
    for t in v {
        t.join().expect("cannot join the thread.");
    }
}
