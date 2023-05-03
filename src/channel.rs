use crate::backend_task::BackendTask;
use crate::time_spent::TimeSpent;
pub mod consumer;
#[cfg(not(target_os = "windows"))]
pub mod kafka_dest;
pub mod producer;

pub enum Message {
    BackendTask(BackendTask),
    TimeSpent(TimeSpent),
    
}
