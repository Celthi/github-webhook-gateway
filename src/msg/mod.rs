pub mod consumer;
pub mod task;
use task::BackendTask;

#[cfg(not(target_os = "windows"))]
pub mod kafka_dest;
pub mod producer;
pub mod queue;
pub mod time_spent;
pub use time_spent::TimeSpent;

pub enum Message {
    BackendTask(BackendTask),
    TimeSpent(TimeSpent),
}
