use chrono::Utc;

use crate::{
    domain::{Task, TaskId},
    error::{Result, TaskError},
    storage::TaskRepository,
};

pub const MAX_TITLE_LEN: usize = 100;

pub struct TaskService<R: TaskRepository> {
    repo: R,
}
impl<R: TaskRepository> TaskService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn validate_title(raw: &str) -> Result<String> {
        let title = raw.trim();

        if title.is_empty() {
            return Err(TaskError::InvalidInput("girdi boş olamaz".to_string()));
        }

        if title.chars().count() > MAX_TITLE_LEN {
            return Err(TaskError::InvalidInput(format!(
                "başlık en fazla {MAX_TITLE_LEN} karakter olabilir."
            )));
        }

        Ok(title.to_string())
    }

    pub fn add(&mut self, title: &str) -> Result<Task> {
        let title = Self::validate_title(title)?;
        let id = self.repo.next_id()?;

        let task = Task::new(id, title, Utc::now());
        self.repo.add(task.clone())?;

        Ok(task)
    }

    pub fn list(&self) -> Result<Vec<Task>> {
        self.repo.list()
    }

    pub fn find(&self, id: TaskId) -> Result<Task> {
        self.repo.get(id)?.ok_or(TaskError::NotFound(id))
    }

    pub fn complete(&mut self, id: TaskId) -> Result<Task> {
        let mut task = self.find(id)?;

        task.complete();
        self.repo.update(task.clone())?;

        Ok(task)
    }

    pub fn delete(&mut self, id: TaskId) -> Result<()> {
        self.repo.delete(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::TaskStatus, storage::in_memory::InMemoryRepository};

    use super::*;

    fn service() -> TaskService<InMemoryRepository> {
        TaskService::new(InMemoryRepository::new())
    }

    #[test]
    fn add() {
        let mut service = service();
        let task = service.add("ekmek al").unwrap();

        assert_eq!(task.id(), TaskId::new(1));
        assert_eq!(task.title(), "ekmek al");
        assert_eq!(task.status(), TaskStatus::Pending);
        assert_eq!(service.list().unwrap().len(), 1);
    }

    #[test]
    fn add_trim() {
        let mut service = service();
        let task = service.add("   ekmek al  ").unwrap();

        assert_eq!(task.title(), "ekmek al");
    }

    #[test]
    fn add_and_compare_task_id() {
        let mut service = service();
        let task1 = service.add("ekmek al").unwrap();
        let task2 = service.add("süt al").unwrap();

        assert_eq!(task1.id(), TaskId::new(1));
        assert_eq!(task2.id(), TaskId::new(2));
    }

    #[test]
    fn reject_empty_title() {
        let mut service = service();

        assert!(matches!(
            service.add("").unwrap_err(),
            TaskError::InvalidInput(_)
        ));
        assert!(matches!(
            service.add(" ").unwrap_err(),
            TaskError::InvalidInput(_)
        ));
    }

    #[test]
    fn reject_long_title() {
        let mut service = service();
        let title = "t".repeat(MAX_TITLE_LEN + 1);

        assert!(matches!(
            service.add(&title).unwrap_err(),
            TaskError::InvalidInput(_)
        ));
    }

    #[test]
    fn accept_exact_len_title() {
        let mut service = service();
        let title = "t".repeat(MAX_TITLE_LEN);

        assert!(service.add(&title).is_ok());
    }

    #[test]
    fn complete() {
        let mut service = service();
        let task = service.add("ekmek al").unwrap();

        let completed = service.complete(task.id()).unwrap();

        assert_eq!(completed.status(), TaskStatus::Completed);
        assert_eq!(service.list().unwrap()[0].status(), TaskStatus::Completed);
    }

    #[test]
    fn completed_not_even_task() {
        let mut service = service();
        let err = service.complete(TaskId::new(99)).unwrap_err();

        assert!(matches!(err, TaskError::NotFound(_)));
    }

    #[test]
    fn delete_task() {
        let mut service = service();
        let task = service.add("t").unwrap();
        service.delete(task.id()).unwrap();

        assert!(service.list().unwrap().is_empty());
    }

    #[test]
    fn delete_not_event_task() {
        let mut service = service();
        let err = service.delete(TaskId::new(99)).unwrap_err();

        assert!(matches!(err, TaskError::NotFound(_)));
    }
}
