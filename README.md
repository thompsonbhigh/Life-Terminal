# Life Terminal

A fast, keyboard-driven personal productivity dashboard for the terminal.

Life Terminal is a Rust TUI intended to become a local-first daily command
center for goals, study tracking, tasks, habits, notes, and other personal
workflows.

> This project is in early development. The current version provides the TUI
> foundation and placeholder sections; productivity data is not stored yet.

## Features

- Native terminal UI built with Ratatui and Crossterm
- Keyboard-first navigation
- Six modular sections: Dashboard, Goals, Study, Tasks, Habits, and Notes
- Direct section switching with number keys
- Safe terminal setup and restoration on exit
- Small section-based architecture for future expansion

## Screenshot

```text
┌ Life Terminal ──────────────────────────────────────┐
│                                                      │
├──────────────────────────────────────────────────────┤
│ [1] Dashboard  [2] Goals  [3] Study  [4] Tasks ...  │
│                                                      │
│ ┌ Dashboard ──────────────────────────────────────┐ │
│ │                                                  │ │
│ │ Dashboard                                        │ │
│ │                                                  │ │
│ │ Section coming soon                              │ │
│ │                                                  │ │
│ └──────────────────────────────────────────────────┘ │
│ Tab: Next  Shift+Tab: Previous  1-6: Switch  q: Quit │
└──────────────────────────────────────────────────────┘
```

## Requirements

For Docker setup, skip to [Run with Docker](#run-with-docker); Rust is only
required for running locally.

- Rust stable, including Cargo
- A terminal with Unicode box-drawing character support

Install Rust with [rustup](https://rustup.rs/) if it is not already available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart the terminal after installation, or load the Cargo environment for the
current shell:

```bash
source "$HOME/.cargo/env"
```

## Run locally

```bash
git clone https://github.com/YOUR-USERNAME/life-terminal.git
cd life-terminal
cargo run
```

## Run with Docker

Install Docker Engine with the Compose plugin on Linux, or Docker Desktop on
macOS/Windows (using Linux containers). From this repository's directory, run:

```bash
docker compose run --rm --build life-terminal
```

Press `q` to exit. Run the same command again to reopen the app. This is an
interactive terminal application, so launch it in a terminal rather than as a
background service. No ports need to be exposed.

The image builds from source for the host architecture, including x86-64 and
ARM64. Rust and build tools are included only in the build stage; the app runs
as a non-root user in the final image.

Your database is saved at `/data/life-terminal.db` in the named
`life-terminal-data` volume managed by Compose. It survives container removal
and image rebuilds. `docker compose down --volumes` deletes this saved data.
The database in your local checkout is not copied into the image, and Docker
volumes do not automatically sync between devices.

Without Compose, use:

```bash
docker build -t life-terminal .
docker run --rm -it --mount source=life-terminal-data,target=/data life-terminal
```

The `-it` flags are required for keyboard input and terminal rendering. The
plain Docker command uses a separate volume from Compose's project-prefixed
volume, so use the same launch method to keep accessing the same data.

## Controls

| Key | Action |
| --- | --- |
| `Tab` | Next section |
| `Shift+Tab` | Previous section |
| `1`–`6` | Open Dashboard, Goals, Study, Tasks, Habits, or Notes |
| `q` | Quit and restore the terminal |

## Architecture

Each primary feature is an independent section. The application owns global
navigation and delegates rendering, updates, and section-specific input to the
active section.

```text
src/
├── main.rs          Terminal setup and cleanup
├── app.rs           Application loop, layout, global navigation
├── event.rs         Event polling
└── sections/
    ├── mod.rs       Section trait and registration
    ├── dashboard.rs
    ├── goals.rs
    ├── study.rs
    ├── tasks.rs
    ├── habits.rs
    └── notes.rs
```

The section interface keeps future features isolated:

```rust
pub trait Section {
    fn name(&self) -> &'static str;
    fn handle_event(&mut self, event: &Event);
    fn update(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect);
}
```

## Development

Before submitting changes, run:

```bash
cargo fmt
cargo test
cargo clippy -- -D warnings
```

## Roadmap

- [x] Terminal initialization and cleanup
- [x] Event loop and global navigation
- [x] Modular placeholder sections
- [ ] SQLite persistence
- [ ] Goals management
- [ ] Study timer and session history
- [ ] Tasks, habits, and notes
- [ ] Dashboard summaries
- [ ] Command palette

## Tech stack

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/)
- [Crossterm](https://github.com/crossterm-rs/crossterm)
- SQLite (planned persistence layer)

## License

No license has been selected yet. Add one before distributing or accepting
outside contributions.
