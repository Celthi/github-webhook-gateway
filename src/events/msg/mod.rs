pub mod task;
use task::Task;
pub mod consumer;
pub mod time_spent;
pub mod queue;
use time_spent::TimeSpent;
pub enum Message {
    Task(Task),
    TimeSpent(TimeSpent),
}