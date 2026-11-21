use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::storage::TaskRepository;
use crate::{
    domain::task::{Task, TaskId},
    error::{Result, TaskError},
};

type TaskMap = BTreeMap<TaskId, Task>;

pub struct JsonRepository {
    path: PathBuf,
    tasks: TaskMap,
}

impl JsonRepository {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let tasks = Self::load(&path)?;

        Ok(Self { path, tasks })
    }

    fn load(path: &Path) -> Result<TaskMap> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(TaskMap::new()),
            Err(e) => return Err(e.into()),
        };

        let list: Vec<Task> = serde_json::from_str(&content)?;
        let mut map = TaskMap::new();

        for task in list {
            let id = task.id();
            if map.insert(id, task).is_some() {
                return Err(TaskError::InvalidInput(format!(
                    "dosyada yinelenen görev id'si: {id}"
                )));
            }
        }
        Ok(map)
    }

    fn write(&self, tasks: &TaskMap) -> Result<()> {
        let list: Vec<&Task> = tasks.values().collect();
        let json = serde_json::to_string_pretty(&list)?;

        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let tmp_path = self.path.with_extension("json.tmp");
        let mut file = File::create(&tmp_path)?;

        file.write_all(json.as_bytes())?;
        file.sync_all()?;

        fs::rename(&tmp_path, &self.path)?;

        Ok(())
    }

    fn commit(&mut self, next: TaskMap) -> Result<()> {
        self.write(&next)?;
        self.tasks = next;

        Ok(())
    }
}

impl TaskRepository for JsonRepository {
    fn next_id(&self) -> Result<TaskId> {
        let next = self.tasks.keys().next_back().map_or(1, |id| id.value() + 1);
        Ok(TaskId::new(next))
    }

    fn add(&mut self, task: Task) -> Result<()> {
        let mut next = self.tasks.clone();
        next.insert(task.id(), task);
        self.commit(next)
    }

    fn get(&self, id: TaskId) -> Result<Option<Task>> {
        Ok(self.tasks.get(&id).cloned())
    }

    fn list(&self) -> Result<Vec<Task>> {
        Ok(self.tasks.values().cloned().collect())
    }

    fn update(&mut self, task: Task) -> Result<()> {
        let id = task.id();

        if !self.tasks.contains_key(&id) {
            return Err(TaskError::NotFound(id));
        }

        let mut next = self.tasks.clone();
        next.insert(id, task);
        self.commit(next)
    }

    fn delete(&mut self, id: TaskId) -> Result<()> {
        let mut next = self.tasks.clone();
        next.remove(&id).ok_or(TaskError::NotFound(id))?;
        self.commit(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use tempfile::tempdir;

    fn get_file_name() -> String {
        "tasks.json".to_string()
    }

    fn sample_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 20, 20, 10, 5).unwrap()
    }

    fn task(id: TaskId, title: &str) -> Task {
        Task::new(id, title.to_string(), sample_time())
    }

    #[test]
    fn missing_file_yields_empty_repository() {
        let dir = tempdir().unwrap();
        let repo = JsonRepository::open(dir.path().join(get_file_name())).unwrap();
        assert!(repo.list().unwrap().is_empty());
        assert_eq!(repo.next_id().unwrap(), TaskId::new(1));
    }

    #[test]
    fn added_tasks_survice_reopen() {
        let dir = tempdir().unwrap();
        let path = dir.path().join(get_file_name());

        let mut repo = JsonRepository::open(&path).unwrap();
        repo.add(task(TaskId::new(1), "birinci")).unwrap();
        repo.add(task(TaskId::new(2), "birinci")).unwrap();

        let reopened = JsonRepository::open(&path).unwrap();
        let tasks = reopened.list().unwrap();

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title(), "birinci");
        assert_eq!(reopened.next_id().unwrap(), TaskId::new(3));
    }

    #[test]
    fn update_and_delete_are_persisted() {
        let dir = tempdir().unwrap();
        let path = dir.path().join(get_file_name());

        let mut repo = JsonRepository::open(&path).unwrap();
        repo.add(task(TaskId::new(1), "a")).unwrap();
        repo.add(task(TaskId::new(2), "b")).unwrap();

        let mut task_one = repo.get(TaskId::new(1)).unwrap().unwrap();
        task_one.complete();
        repo.update(task_one).unwrap();
        repo.delete(TaskId::new(2)).unwrap();

        let reopened = JsonRepository::open(&path).unwrap();
        let tasks = reopened.list().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status(), crate::domain::TaskStatus::Completed);
    }

    #[test]
    fn update_and_delete_missing_task_return_not_found() {
        let dir = tempdir().unwrap();
        let mut repo = JsonRepository::open(dir.path().join(get_file_name())).unwrap();

        assert!(matches!(
            repo.update(task(TaskId::new(9), "yok")),
            Err(TaskError::NotFound(_))
        ));
        assert!(matches!(
            repo.delete(TaskId::new(9)),
            Err(TaskError::NotFound(_))
        ));
    }

    #[test]
    fn corrupt_file_is_reported_and_left_untouched() {
        let dir = tempdir().unwrap();
        let path = dir.path().join(get_file_name());
        fs::write(&path, "{ bu json değil").unwrap();

        let result = JsonRepository::open(&path);

        assert!(matches!(result, Err(TaskError::Serialization(_))));
        assert_eq!(fs::read_to_string(&path).unwrap(), "{ bu json değil");
    }

    #[test]
    fn duplicate_ids_in_file_are_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join(get_file_name());
        let json =
            serde_json::to_string(&vec![task(TaskId::new(1), "a"), task(TaskId::new(1), "b")])
                .unwrap();
        fs::write(&path, json).unwrap();

        assert!(matches!(
            JsonRepository::open(&path),
            Err(TaskError::InvalidInput(_))
        ));
    }

    #[test]
    fn no_temp_file_is_left_behind() {
        let dir = tempdir().unwrap();
        let path = dir.path().join(get_file_name());
        let mut repo = JsonRepository::open(&path).unwrap();
        repo.add(task(TaskId::new(1), "a")).unwrap();

        assert!(path.exists());
        assert!(!dir.path().join("tasks.json.tmp").exists());
    }

    #[test]
    fn creates_missing_parent_directories() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("data").join(get_file_name());
        let mut repo = JsonRepository::open(&path).unwrap();
        repo.add(task(TaskId::new(1), "a")).unwrap();
        assert!(path.exists());
    }
}
