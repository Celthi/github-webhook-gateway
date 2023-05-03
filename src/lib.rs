pub mod config_env;
pub mod github;
pub mod backend_task;
#[macro_use]
extern crate colonbuilder;
pub mod constants;
pub mod channel;
pub mod queue;
pub mod web;
pub mod time_spent;
pub mod event;
macro_rules! reg {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub(crate) use reg;
