pub mod commands;

use commands::Command;
use std::io::Write;

use crate::{
    domain::{Task, TaskId, TaskStatus},
    error::Result,
    service::TaskService,
    storage::TaskRepository,
};

/// bir komut çalıştır ve çıktıyı `out` 'a gönder
pub fn execute<R: TaskRepository>(
    service: &mut TaskService<R>,
    command: Command,
    out: &mut impl Write,
) -> Result<()> {
    match command {
        Command::Add { title } => {
            let task = service.add(&title)?;
            writeln!(
                out,
                "{} görev olarak eklendi.",
                format_task(&task)
            )?;
        }
        Command::List => {
            let tasks = service.list()?;
            if tasks.is_empty() {
                writeln!(out, "Henüz görev yok.")?;
            }

            for task in tasks {
                writeln!(out, "{}", format_task(&task))?;
            }
        }
        Command::Done { id } => {
            let task = service.complete(TaskId::new(id))?;
            writeln!(out, "{} görev tamamlandı.", format_task(&task))?;
        }
        Command::Delete { id } => {
            service.delete(TaskId::new(id))?;
            writeln!(out, "#{} numaralı görev silindi.", TaskId::new(id))?;
        }
    }
    Ok(())
}

fn format_task(task: &Task) -> String {
    format!("{:?}", &task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::InMemoryRepository;

    fn service() -> TaskService<InMemoryRepository> {
        TaskService::new(InMemoryRepository::default())
    }

    fn run(service: &mut TaskService<InMemoryRepository>, command: Command) -> String {
        let mut out = Vec::new();
        execute(service, command, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn add_prints_created_task() {
        let mut service = service();
        let out = run(
            &mut service,
            Command::Add {
                title: "ekmek al".into(),
            },
        );
        assert!(out.contains("Eklendi"));
        assert!(out.contains("ekmek al"));
    }

    #[test]
    fn list_on_empty_repository_says_so() {
        let mut service = service();
        let out = run(&mut service, Command::List);
        assert!(out.contains("Henüz görev yok."));
    }

    #[test]
    fn done_marks_task_in_output() {
        let mut service = service();
        run(
            &mut service,
            Command::Add {
                title: "sut al".into(),
            },
        );
        let out = run(&mut service, Command::Done { id: 1 });
        assert!(out.contains("Tamamlandı"));
    }

    #[test]
    fn delete_unknown_id_returns_not_found() {
        let mut service = service();
        let mut out = Vec::new();
        let result = execute(&mut service, Command::Delete { id: 99 }, &mut out);
        assert!(matches!(result, Err(crate::error::TaskError::NotFound(_))));
    }
}
