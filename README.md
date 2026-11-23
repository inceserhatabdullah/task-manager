# Task Manager

- komut satırından görev ekleme, listeleme, silme ve tamamlama.

## Kullanım
- görev ekleme
```sh
cargo run -- add "ekmek al"
```

```json
Task { id: TaskId(1), title: "ekmek al", status: Pending, created_at: 2026-11-23T16:36:03.003305Z } eklendi.
```

- görev listeleme
```sh
cargo run -- list
```

- görev tamamlama
```sh
cargo run -- done 1
```

```json
Task { id: TaskId(1), title: "ekmek al", status: Completed, created_at: 2026-11-23T16:36:03.003305Z } tamamlandı.
```

- görev silme
```sh
cargo run -- delete 1
```

```json
Task { id: TaskId(1), title: "ekmek al", status: Pending, created_at: 2026-11-23T16:36:03.003305Z } silindi.
```
