use crate::constants;
use crate::msg;
use crate::rally;
use poem::{handler, web::Json};
use serde_json;
use tracing::error;
#[handler]
pub fn process(req: String) -> Json<serde_json::Value> {
    if constants::contains_rally_pattern(&req) {
        match rally::Event::new(&req) {
            Ok(e) => {
                if let Err(e) =
                    msg::producer::handle_rally_message(&e, Some("Review and Support".to_string()))
                {
                    error!("Cannot process rally message, error: {}{:?}", req, e);
                    return Json(serde_json::json! ({
                    "code": 0,
                    "message": "Finish processing github event"}));
                }
            }
            Err(e) => {
                error!("Cannot process rally message, error: {}{:?}", req, e);
            }
        }
    }
    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing rally event"}))
}
