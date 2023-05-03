use crate::constants;
use crate::github::event::GithubEvent;
use crate::msg;
use crate::msg::queue;
use crate::msg::time_spent;
use anyhow::Result;

use super::time_spent::TimeSpentTrait;

pub fn handle_github_message(event: &GithubEvent) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    if let Some(comment) = event.get_code() {
        let mut msg;
        if constants::contains_time_spent_pattern(comment) && event.get_action() != "edited" {
            // somehow submit a review will create two events: edited and submitted, only care the 'submitted' event only.
            if let Some(tp) = time_spent::get_time_spent(comment, event, None) {
                msg = msg::Message::TimeSpent(tp);
                let s = queue::get_sender();
                let guard = s.lock();
                let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
                sender.send(msg)?;
            }
        }
        if constants::contains_ocr_patten(comment) {
            if let (Some(repo), Some(pr)) = (event.get_repo_name(), event.get_pr_number()) {
                let task = msg::task::get_task_from_str(
                    comment,
                    repo.to_string(),
                    pr,
                    event.get_user_name(),
                )?;
                msg = msg::Message::Task(task);
                let s = queue::get_sender();
                let guard = s.lock();
                let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
                sender.send(msg)?;
            }
        }
    }

    Ok(())
}
