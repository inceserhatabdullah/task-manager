use assert_cmd::Command;
use tempfile::tempdir;

fn cmd(file: &std::path::Path) -> Command {
    let mut command = Command::cargo_bin("task-manager").unwrap();
    command.arg("--file").arg(file);
    command
}

#[test]
fn tasks_persist_between_invovations() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("tasks.json");

    cmd(&file).args(["add", "kitap oku"]).assert().success();
    cmd(&file)
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("kitap oku"));
}

#[test]
fn unknown_id_fails_with_message_on_stderr() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("tasks.json");

    cmd(&file)
        .args(["done", "99"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("bulunamadı"));
}
