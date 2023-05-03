use crate::constants;
use crate::events::github;
use crate::events;

use crate::events::msg::time_spent::TimeSpentTrait;
use poem::{handler, web::Json};
use serde_json;
use tracing::error;

#[handler]
pub fn process(req: String) -> Json<serde_json::Value> {
    if !constants::contain_keywords(&req) {
        return Json(serde_json::json! ({
            "code": 0,
            "message": "Not interested.",
        }));
    }

    let event = match github::event::GithubEvent::new(&req) {
        Ok(event) => event,
        Err(e) => {
            error!(
                "Cannot process github message, error: {:?}\n. payload is:\n{}",
                e, req
            );
            return Json(serde_json::json! ({
            "code": 0,
            "message": "Cannot process github message"}));
        }
    };
    if let Err(e) = events::github::handler::handle_github_event(&event) {
        error!("Cannot process github message, error: {:?}", e);
        if let (Some(repo), Some(pr)) = (
            event.get_repo_name().map(str::to_string),
            event.get_pr_number(),
        ) {
            tokio::spawn(async move {
                let _ = github::post_issue_comment(&repo, pr, &format!("{}", e)).await;
            });
        }
    }

    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing github event"}))
}
