use crate::constants;
use crate::events::rally;
use poem::{handler, web::Json};
use serde_json;
use tracing::error;

#[handler]
pub fn process(req: String) -> Json<serde_json::Value> {
    if !constants::contains_rally_pattern(&req) {
        return Json(serde_json::json! ({
            "code": 0,
            "message": "Not interested.",
        }));
    }
    match rally::Event::new(&req) {
        Ok(e) => {
            if let Err(e) =
                rally::handler::handle_rally_event(&e, Some("Review and Support".to_string()))
            {
                error!("Cannot handle rally message, error: {}{:?}", req, e);
                return Json(serde_json::json! ({
                    "code": 0,
                    "message": "Finish processing rally event"}));
            }
        }
        Err(e) => {
            error!("Not valid rally message, error: {}{:?}", req, e);
        }
    }

    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing rally event"}))
}
