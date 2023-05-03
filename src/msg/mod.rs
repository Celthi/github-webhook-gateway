pub mod consumer;
pub mod task;
use task::Task;
pub mod queue;
pub mod time_spent;
pub use time_spent::TimeSpent;
pub mod github;
pub mod rally;
pub enum Message {
    Task(Task),
    TimeSpent(TimeSpent),
}
