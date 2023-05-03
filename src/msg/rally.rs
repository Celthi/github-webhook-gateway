use crate::constants;
use crate::msg;
use crate::msg::queue;
use crate::msg::time_spent;
use anyhow::Result;
use super::time_spent::TimeSpentTrait;

pub fn handle_rally_message<T: TimeSpentTrait>(event: &T, task_name: Option<String>) -> Result<()> {
    if let Some(comment) = event.get_code() {
        if !constants::contains_time_spent_pattern(comment) {
            return Ok(());
        }
        if let Some(tp) = time_spent::get_time_spent(comment, event, task_name) {
            let msg = msg::Message::TimeSpent(tp);
            let s = queue::get_sender();
            let guard = s.lock();
            let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
            sender.send(msg)?;
        }
    }

    Ok(())
}
