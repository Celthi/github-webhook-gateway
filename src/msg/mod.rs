pub mod consumer;
pub mod task;
use task::Task;
pub mod producer;
pub mod queue;
pub mod time_spent;
pub use time_spent::TimeSpent;

pub enum Message {
    Task(Task),
    TimeSpent(TimeSpent),
}
