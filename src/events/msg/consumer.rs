use super::time_spent;
use crate::events::msg::queue;
use crate::events::msg::task;
use crate::events::msg::Message;
use tokio;

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
