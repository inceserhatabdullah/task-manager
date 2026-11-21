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
            writeln!(out, "Eklendi: {}", format_task(&task))?;
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
            writeln!(out, "Tamamlandı: {}", TaskId::new(id))?;
        }
        Command::Delete { id } => {
            service.delete(TaskId::new(id))?;
            writeln!(out, "Silindi: {}", TaskId::new(id))?;
        }
    }
    Ok(())
}

fn format_task(task: &Task) -> String {
    let mark = match task.status() {
        TaskStatus::Completed => "x",
        TaskStatus::Pending => "",
    };

    format!("[{mark}] {} {}", task.id(), task.title())
}
