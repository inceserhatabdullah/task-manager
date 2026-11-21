use std::collections::BTreeMap;

use crate::{
    domain::{Task, TaskId},
    error::{Result, TaskError},
    storage::TaskReposiyory,
};

#[derive(Debug, Default)]
pub struct InMemoryRepository {
    tasks: BTreeMap<TaskId, Task>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TaskReposiyory for InMemoryRepository {
    fn next_id(&self) -> Result<TaskId> {
        // son anahtar en büyüktür. (btreemap sıralı) Boş ise 1'den başla.
        let next = self.tasks.keys().next_back().map_or(1, |id| id.value() + 1);
        Ok(TaskId::new(next))
    }

    fn add(&mut self, task: Task) -> Result<()> {
        self.tasks.insert(task.id(), task);
        Ok(())
    }

    fn get(&self, id: TaskId) -> Result<Option<Task>> {
        Ok(self.tasks.get(&id).cloned())
    }

    fn list(&self) -> Result<Vec<Task>> {
        Ok(self.tasks.values().cloned().collect())
    }

    fn update(&mut self, task: Task) -> Result<()> {
        match self.tasks.get_mut(&task.id()) {
            Some(existing) => {
                *existing = task;
                Ok(())
            }
            None => Err(TaskError::NotFound(task.id())),
        }
    }

    fn delete(&mut self, id: TaskId) -> Result<()> {
        self.tasks
            .remove(&id)
            .map(|_| ())
            .ok_or(TaskError::NotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    use super::*;
    use crate::domain::TaskStatus;

    fn sample_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 20, 20, 10, 5).unwrap()
    }

    fn task(id: u64, title: &str) -> Task {
        Task::new(TaskId::new(id), title.to_string(), sample_time())
    }

    #[test]
    fn first_id_is_one_in_empty_repo() {
        let repo = InMemoryRepository::new();
        assert_eq!(repo.next_id().unwrap(), TaskId::new(1));
    }

    #[test]
    fn next_id_returns_max_id_plus_one() {
        let mut repo = InMemoryRepository::new();
        repo.add(task(1, "ekmek al")).unwrap();
        repo.add(task(5, "süt al")).unwrap();

        assert_eq!(repo.next_id().unwrap(), TaskId::new(6));
    }

    #[test]
    fn added_task_can_be_retrieved() {
        let mut repo = InMemoryRepository::new();
        let task = task(1, "ekmek al");
        repo.add(task.clone()).unwrap();

        assert_eq!(repo.get(TaskId::new(1)).unwrap(), Some(task));
    }

    #[test]
    fn nonexistent_task_returns_none() {
        let repo = InMemoryRepository::new();
        assert_eq!(repo.get(TaskId::new(99)).unwrap(), None);
    }

    #[test]
    fn list_returns_tasks_sorted_by_id() {
        let mut repo = InMemoryRepository::new();
        repo.add(task(3, "ekmek al")).unwrap();
        repo.add(task(1, "süt al")).unwrap();
        repo.add(task(2, "gazete al")).unwrap();

        let ids: Vec<u64> = repo
            .list()
            .unwrap()
            .iter()
            .map(|t| t.id().value())
            .collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn update_modifies_task() {
        let mut repo = InMemoryRepository::new();
        repo.add(task(1, "ekmek al")).unwrap();

        let mut task = repo.get(TaskId::new(1)).unwrap().unwrap();
        task.complete();
        repo.update(task).unwrap();

        let record = repo.get(TaskId::new(1)).unwrap().unwrap();
        assert_eq!(record.status(), TaskStatus::Completed);
    }

    #[test]
    fn updating_nonexistent_task_returns_not_found() {
        let mut repo = InMemoryRepository::new();
        let err = repo.update(task(42, "ekmek al")).unwrap_err();

        assert!(matches!(err, TaskError::NotFound(id) if id == TaskId::new(42)));
    }

    #[test]
    fn delete_removes_task() {
        let mut repo = InMemoryRepository::new();
        repo.add(task(1, "ekmek al")).unwrap();
        repo.delete(TaskId::new(1)).unwrap();

        assert_eq!(repo.get(TaskId::new(1)).unwrap(), None);
    }

    #[test]
    fn deleting_nonexistent_task_returns_not_found() {
        let mut repo = InMemoryRepository::new();
        let err = repo.delete(TaskId::new(7)).unwrap_err();

        assert!(matches!(err, TaskError::NotFound(_)));
    }
}
