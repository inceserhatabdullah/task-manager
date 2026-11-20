use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Completed,
}

/// Alanlar `pub` değil, doğrudan değiştirilemez. Değişiklik method üzerinden yapılır.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    title: String,
    status: TaskStatus,
    created_at: DateTime<Utc>,
}

impl Task {
    /// yeni bir görev oluştur
    /// `created_at` dışarıdan alınıyor. Nedeni; `utc.now()` ile içeride alırsak testler her çalıştığında farklı sonuç verir.
    /// zamanı parametre olarak almak testleri deterministik yapar.
    pub fn new(id: TaskId, title: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            status: TaskStatus::Pending,
            created_at,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    /// görevi tamamlandı olarak işaretler
    /// `&mut self` aldığı için yalnızca değiştirilebilir bir referansla çağırılabilir.
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 20, 20, 10, 5).unwrap()
    }

    #[test]
    fn new_tast_starts_as_pending() {
        let task = Task::new(TaskId::new(1), "ekmek al".to_string(), sample_time());

        assert_eq!(task.status(), TaskStatus::Pending);
        assert!(!task.is_completed());
    }

    #[test]
    fn complete_marks_task_as_completed() {
        let mut task = Task::new(TaskId::new(1), "ekmek al".to_string(), sample_time());

        task.complete();

        assert_eq!(task.status(), TaskStatus::Completed);
        assert!(task.is_completed());
    }

    #[test]
    fn task_keeps_its_fields() {
        let task = Task::new(TaskId::new(1), "test".to_string() , sample_time());

       assert_eq!(task.id(), TaskId::new(1));
       assert_eq!(task.title(), "test");
       assert_eq!(task.created_at(), sample_time());
    }

    #[test]
    fn task_survives_json_roundtrip() {
        let original = Task::new(TaskId::new(1), "test".to_string() , sample_time());

        let json = serde_json::to_string(&original).unwrap();
        let restored: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(original, restored);
    }
}
