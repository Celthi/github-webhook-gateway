use crate::constants;
use crate::events::github::event::GithubEvent;
use crate::events;
use crate::events::msg::time_spent;
use crate::events::msg::time_spent::TimeSpentTrait;
use crate::events::msg::queue;
use anyhow::Result;

pub fn handle_github_event(event: &GithubEvent) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    if let Some(comment) = event.get_code() {
        // somehow submit a review will create two events: edited and submitted, only care the 'submitted' event only.
        if constants::contains_time_spent_pattern(comment) && event.get_action() != "edited" {
            handle_time_spent_event(comment, event)?;
        }
        if constants::contains_ocr_patten(comment) {
            handle_ocr_event(event, comment)?;
        }
    }
    Ok(())
}

fn handle_ocr_event(event: &GithubEvent, comment: &str) -> Result<(), anyhow::Error> {
    if let (Some(repo), Some(pr)) = (event.get_repo_name(), event.get_pr_number()) {
        let task = events::msg::task::get_task_from_str(
            comment,
            repo.to_string(),
            pr,
            event.get_user_name(),
        )?;
        let msg = events::msg::Message::Task(task);
        let s = queue::get_sender();
        let guard = s.lock();
        let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
        sender.send(msg)?;
    }
    Ok(())
}

fn handle_time_spent_event(comment: &str, event: &GithubEvent) -> Result<()> {
    if let Some(tp) = time_spent::get_time_spent(comment, event, None) {
        let msg = events::msg::Message::TimeSpent(tp);
        let s = queue::get_sender();
        let guard = s.lock();
        let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
        sender.send(msg)?;
    }
    Ok(())
}
