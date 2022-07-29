use crate::backend_task::Task;
pub mod consumer;
pub mod producer;
pub struct Message {
    pub backend_task: Task,
}
