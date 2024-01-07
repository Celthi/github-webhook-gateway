use crate::constants;
use crate::events;
use crate::events::github;
use crate::events::msg::time_spent::TimeSpentTrait;
use tracing::error;

pub async fn process(req: String) -> &'static str {
    if !constants::contain_keywords(&req) {
        return "not interested";
    }

    let event = match github::event::GithubEvent::new(&req) {
        Ok(event) => event,
        Err(e) => {
            error!(
                "Cannot process github message, error: {:?}\n. payload is:\n{}",
                e, req
            );
            return "Cannot process github message";
        }
    };
    tokio::spawn(async move {
        let user = match github::get_user_name(event.get_login_name()).await {
            Ok(n) => n,
            Err(e) => {
                error!("get user name error:{}", e);
                return;
            }
        };
        if let Err(e) = events::github::handler::handle_github_event(&event, &user) {
            error!("Cannot process github message, error: {:?}", e);
            if let (Some(repo), Some(pr)) = (
                event.get_repo_name().map(str::to_string),
                event.get_pr_number(),
            ) {
                let _ = github::post_issue_comment(&repo, pr, &format!("{}", e)).await;
            }
        }
    });

    "Finish processing github event"
}
