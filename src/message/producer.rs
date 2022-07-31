use crate::backend_task;
use crate::github::event::GithubEvent;
use crate::message;
use crate::queue;
use crate::constants;
use anyhow::Result;
use tracing::{info, error};

pub fn produce_message_from(event: &GithubEvent) -> Result<()> {
    let action = event.get_action();
    if action == "deleted" {
        return Ok(());
    }
    let comment = event.get_comment();
    if !constants::contains_keywords_we_focus(&comment) {
        return Ok(());
    }
    match backend_task::get_backend_task_from_str(
        &comment,
        &event.get_repo_name(),
        event.get_pr_number(),
        event.get_user_name(),
    ) {
        Ok(backend_task) => {
            let s = queue::get_sender();
            let guard = s.lock();
            let sender = guard.expect("get sender fail."); // crash here if the channel is malfunc
            if sender.send(message::Message { backend_task }).is_err() {
                error!("Fail to send a task to channel.");
            }
        }
        Err(e) => {
            error!("Cannot get backend task from body.{:?}", e);
        }
    }
    Ok(())
}
