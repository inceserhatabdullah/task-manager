pub mod in_memory;
pub mod json_repository;

use crate::domain::{Task, TaskId};
use crate::error::Result;

pub trait TaskRepository {
    fn next_id(&self) -> Result<TaskId>;
    fn add(&mut self, task: Task) -> Result<()>;
    fn get(&self, id: TaskId) -> Result<Option<Task>>;
    fn list(&self) -> Result<Vec<Task>>;
    fn update(&mut self, task: Task) -> Result<()>;
    fn delete(&mut self, id: TaskId) -> Result<()>;
}

pub use json_repository::{JsonRepository};
pub use in_memory::{InMemoryRepository};
