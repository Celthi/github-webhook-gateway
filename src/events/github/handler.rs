use crate::constants;
use crate::events::github::event::GithubEvent;
use crate::events::github::user::User;
use crate::events::msg::queue;
use crate::events::msg::task;
use crate::events::msg::time_spent;
use crate::events::msg::time_spent::TimeSpentTrait;
use crate::events::msg::Message;
use anyhow::anyhow;
use anyhow::Result;

pub fn handle_github_event(event: &GithubEvent, user: &User) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    let comment = event
        .get_code()
        .ok_or(anyhow!("No comment in the github event"))?;
    // somehow submit a review will create two events: edited and submitted, only care the 'submitted' event only.
    if constants::contains_time_spent_pattern(comment) && event.get_action() != "edited" {
        handle_time_spent_event(event, user, comment)?;
    } else if constants::contains_ocr_patten(comment) {
        handle_ocr_event(event, &user, comment)?;
    }

    Ok(())
}

fn handle_ocr_event(event: &GithubEvent, user: &User, comment: &str) -> Result<(), anyhow::Error> {
    let (Some(repo), Some(pr)) = (event.get_repo_name(), event.get_pr_number())  else { return Ok(());};
    let task = task::get_task_from_str(
        comment,
        repo.to_string(),
        pr,
        user.name.clone().unwrap_or(event.get_user_name()),
    )?;
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    sender.send(Message::Task(task))?;
    Ok(())
}

fn handle_time_spent_event(event: &GithubEvent, user: &User, comment: &str) -> Result<()> {
    let Some(tp) = time_spent::get_time_spent(comment, event, user.email.clone(), None) else { return Ok(());};
    let s = queue::get_sender();
    let guard = s.lock();
    let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
    sender.send(Message::TimeSpent(tp))?;
    Ok(())
}
