use super::time_spent::TimeSpentTrait;
use crate::msg;
use crate::msg::queue;
use crate::msg::time_spent;
use anyhow::Result;
use anyhow::bail;

pub fn handle_rally_message<T: TimeSpentTrait>(event: &T, task_name: Option<String>) -> Result<()> {
    let Some(comment) = event.get_code() else { return Ok(());};
    let Some(tp)= time_spent::get_time_spent(comment, event, task_name) else { return Ok(());};
    let msg = msg::Message::TimeSpent(tp);
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    if let Err(e) = sender.send(msg) {
        bail!("{:?}", e);
    }
    Ok(())
}
