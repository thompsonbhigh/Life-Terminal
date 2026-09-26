# Life Terminal

Life Terminal is a keyboard-driven personal productivity app for the terminal. It is written in Rust with Ratatui and stores tasks, goals, and habits in a local SQLite database.

## Current features

- Add, complete, and delete tasks and habits
- Add goals and subtasks, track completion, and delete entries
- Navigate six sections: Dashboard, Goals, Study, Tasks, Habits, and Notes
- Keep data between runs in `life-terminal.db`

Study and Notes currently show placeholder screens. The Dashboard is a basic overview; this is still an early-stage app.

## Run locally

Install a Rust toolchain with Cargo, then from this directory run:

```bash
cargo run
```

The app creates `life-terminal.db` in the working directory. Run it in an interactive terminal with Unicode support.

## Run with Docker

```bash
docker compose run --rm --build life-terminal
```

Compose runs the terminal interactively and stores the database in the named `life-terminal-data` volume at `/data/life-terminal.db`. Reusing the same Compose project preserves that data; `docker compose down --volumes` removes the volume. No network ports are needed.

## Keyboard controls

| Key | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | Next / previous section |
| `1`–`6` | Open a section directly |
| `j` / `k` or arrow keys | Move selection in a list |
| `a` | Add a task, goal, or habit in its section |
| `A` | Add a subtask while viewing a goal |
| `Enter` | Save an entry or open a goal's subtasks |
| `Space` | Toggle the selected entry |
| `d` | Delete the selected entry |
| `Esc` | Cancel an entry or leave subtask view |
| `q` | Quit from the main view |

While typing in a popup, ordinary keys are treated as text; `Enter` saves and `Esc` cancels.

## Development

`src/app.rs` owns navigation and rendering, `src/sections/` contains the individual screens, and `src/db.rs` creates and queries the SQLite tables. Run `cargo test`, `cargo fmt --check`, and `cargo clippy -- -D warnings` to check changes.
