use crate::msg::Message;
use crate::msg::queue;
use crate::msg::task;
use tokio;

use super::time_spent;

#[tokio::main]
pub async fn event_loop() {
    loop {
        let r = queue::get_receiver();
        let msg;
        {
            let guard = r.lock().expect("cannot get lock from receiver.");
            msg = guard.recv().expect("get message failed from receiver.");
        }
        match msg {
            Message::TimeSpent(tp) => {
                time_spent::handle_time_spent(tp).await;
            }
            Message::Task(task) => {
                task::handle_task(task).await;
            }
        }
    }
}

