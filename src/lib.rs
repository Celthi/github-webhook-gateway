pub mod config_env;
macro_rules! reg {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub(crate) use reg;
#[macro_use]
extern crate colonbuilder;
pub mod constants;
#[cfg(not(target_os = "windows"))]
pub mod kafka;
pub mod events;
pub mod web;

