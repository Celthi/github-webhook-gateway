use crate::channel;
use crate::constants;
use crate::event;
use crate::github::event::GithubEvent;
use crate::queue;
use crate::time_spent;
use crate::backend_task;
use anyhow::Result;

pub fn produce_message_from(event: &GithubEvent) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    if let Some(comment) = event.get_code() {
        if !constants::contain_keywords(comment) {
            return Ok(());
        }
        let mut msg;
        if constants::contains_time_spent_pattern(comment) && event.get_action() != "edited" {
            // somehow submit a review will create two events: edited and submitted, only care the 'submitted' event only.
            if let Some(tp) = time_spent::get_time_spent_from_str(comment, event) {
                msg = channel::Message::TimeSpent(tp);
                let s = queue::get_sender();
                let guard = s.lock();
                let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
                sender.send(msg)?;
            }
        }
        if constants::contains_ocr_patten(comment) {
            if let (Some(repo), Some(pr)) = (event.get_repo_name(), event.get_pr_number()) {
                let task = backend_task::get_backend_task_from_str(
                    comment,
                    repo.to_string(),
                    pr,
                    event.get_user_name(),
                )?;
                msg = channel::Message::BackendTask(task);
                let s = queue::get_sender();
                let guard = s.lock();
                let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
                sender.send(msg)?;
            }
        }
    }

    Ok(())
}

pub fn produce_msg_from(event: &event::rally::Event) -> Result<()> {
    if let Some(comment) = event.get_code() {
        if !constants::contains_time_spent_pattern(comment) {
            return Ok(());
        }
        if let Some(tp) = time_spent::get_time_spent_from_rally_str(comment, event) {
            let msg = channel::Message::TimeSpent(tp);
            let s = queue::get_sender();
            let guard = s.lock();
            let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
            sender.send(msg)?;
        }
    }

    Ok(())
}
