use crate::handler::msg::time_spent::TimeSpentTrait;
use crate::handler;
use crate::handler::queue;
use crate::handler::msg::time_spent;
use anyhow::Result;

pub fn handle_rally_event<T: TimeSpentTrait>(event: &T, task_name: Option<String>) -> Result<()> {
    let Some(comment) = event.get_code() else { return Ok(());};
    let Some(tp)= time_spent::get_time_spent(comment, event, task_name) else { return Ok(());};
    let msg = handler::msg::Message::TimeSpent(tp);
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    sender.send(msg)?;
    Ok(())
}
