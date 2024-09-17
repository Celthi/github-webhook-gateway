use crate::events;
use crate::events::msg::queue;
use crate::events::msg::time_spent;
use crate::events::msg::time_spent::TimeSpentTrait;
use anyhow::Result;
use anyhow::anyhow;

pub fn handle_rally_event<T: TimeSpentTrait>(event: &T, task_name: Option<String>) -> Result<()> {
    let Some(comment) = event.get_code() else {
        return Ok(());
    };
    let Some(tp) = time_spent::get_time_spent(
        comment,
        event,
        Some(event.get_login_name().to_string()),
        task_name,
        Some("rally".to_string()),
    ) else {
        return Ok(());
    };
    let mut res = tp.into_iter().map(|tp| {
        let msg = events::msg::Message::TimeSpent(tp);
        let s = queue::get_sender();
        let guard = s.lock();
        let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
        sender.send(msg)
    });
    if res.all(|r| r.is_err()) {
        return Err(anyhow!("send time spent fail"));
    }
    Ok(())
}
