use crate::constants;
use crate::events::rally;
use tracing::error;

pub async fn process(req: String) -> &'static str {
    if !constants::contains_rally_pattern(&req) {
        return "not interested";
    }
    match rally::Event::new(&req) {
        Ok(e) => {
            if e.get_action() != "Created" {
                return "not interested action";
            }
            if let Err(e) = rally::handler::handle_rally_event(&e, None) {
                error!("Cannot handle rally message, error: {}{:?}", req, e);
                return "Finish processing rally event";
            }
        }
        Err(e) => {
            error!("Not valid rally message, error: {}{:?}", req, e);
        }
    }

    "Finish processing rally event"
}
