pub mod task;
use task::Task;
pub mod consumer;
pub mod queue;
pub mod time_spent;
use time_spent::TimeSpent;
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Task(Task),
    TimeSpent(TimeSpent),
}
