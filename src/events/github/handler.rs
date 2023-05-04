use crate::constants;
use crate::events::github::event::GithubEvent;
use crate::events::msg::queue;
use crate::events::msg::task;
use crate::events::msg::time_spent;
use crate::events::msg::time_spent::TimeSpentTrait;
use crate::events::msg::Message;
use anyhow::anyhow;
use anyhow::Result;

pub fn handle_github_event(event: &GithubEvent, name: Option<String>) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    let comment = event
        .get_code()
        .ok_or(anyhow!("No comment in the github event"))?;
    // somehow submit a review will create two events: edited and submitted, only care the 'submitted' event only.
    if constants::contains_time_spent_pattern(comment) && event.get_action() != "edited" {
        handle_time_spent_event(comment, event, name)?;
    } else if constants::contains_ocr_patten(comment) {
        handle_ocr_event(event, comment, name)?;
    }

    Ok(())
}

fn handle_ocr_event(
    event: &GithubEvent,
    comment: &str,
    name: Option<String>,
) -> Result<(), anyhow::Error> {
    let (Some(repo), Some(pr)) = (event.get_repo_name(), event.get_pr_number())  else { return Ok(());};
    let task = task::get_task_from_str(
        comment,
        repo.to_string(),
        pr,
        name.or(Some(event.get_user_name())).unwrap(),
    )?;
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    sender.send(Message::Task(task))?;
    Ok(())
}

fn handle_time_spent_event(comment: &str, event: &GithubEvent, name: Option<String>) -> Result<()> {
    let Some(tp) = time_spent::get_time_spent(comment, event, name, None) else { return Ok(());};
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    sender.send(Message::TimeSpent(tp))?;
    Ok(())
}
