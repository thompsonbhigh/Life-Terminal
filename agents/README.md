# Life Terminal

Life Terminal is a personal terminal-based productivity application designed to act as a daily command center.

The goal is to provide one fast, keyboard-driven interface for managing everyday activities such as goals, study sessions, tasks, habits, notes, and future productivity tools.

The application is built to be modular so new sections can be added without restructuring the entire program.

## Goals

Life Terminal should be:

* Fast
* Keyboard-driven
* Simple to navigate
* Easy to extend
* Local-first
* Useful for everyday use
* Visually clean while remaining terminal-native

The long-term goal is for Life Terminal to feel more like a personal terminal operating system than a traditional todo application.

## Planned Features

### Dashboard

A central overview of the day.

Possible information includes:

* Today's goals
* Tasks remaining
* Study time
* Habit progress
* Recent activity
* Quick actions

### Goals

Create and manage goals for the current day.

Potential features:

* Add goals
* Complete goals
* Delete goals
* Set priorities
* View previous days
* Daily completion percentage

### Study Tracker

Track time spent studying.

Potential features:

* Start a study session
* Stop a study session
* Select a subject
* Track total time today
* Track weekly and monthly totals
* View study history
* View statistics by subject

Example:

```text
Study
────────────────────────

Today          2h 34m
This Week     11h 12m
This Month    42h 51m

Current Session
CSCE 313

00:42:17
```

### Tasks

General task management.

Potential features:

* Create tasks
* Complete tasks
* Delete tasks
* Set due dates
* Set priorities
* Filter completed tasks

### Habits

Track recurring habits.

Potential features:

* Create habits
* Mark daily completion
* Track streaks
* Display weekly progress
* View habit history

### Notes

Simple terminal-based notes.

Potential features:

* Create notes
* Edit notes
* Search notes
* Organize notes by category

## Future Sections

The architecture should allow additional sections to be added easily.

Possible future sections include:

* Workouts
* Projects
* Calendar
* Finance tracking
* Pomodoro timer
* Reading tracker
* Journal
* Sleep tracking
* GitHub activity
* Weather
* Class schedule

## Command Palette

Life Terminal may include a command palette opened with:

```text
Ctrl + P
```

Example:

```text
╭──────────── Command ────────────╮
│ >                               │
│                                 │
│ Add goal                        │
│ Start study session             │
│ Add task                        │
│ Mark goal complete              │
│ Open study statistics           │
│ Open settings                   │
╰─────────────────────────────────╯
```

Commands could eventually support a command-style syntax:

```text
:goal Finish operating systems homework
:study start CSCE313
:study stop
:task Buy groceries
:open habits
```

## Tech Stack

Life Terminal is written in Rust.

Core technologies:

* Rust
* Ratatui
* Crossterm
* SQLite
* rusqlite
* serde
* chrono
* TOML

Potential future dependencies may include:

* tokio
* thiserror
* anyhow

Dependencies should only be added when they provide clear value.

## Architecture

The application should use a modular section-based architecture.

Each major tab should behave as an independent section of the application.

Conceptually:

```rust
pub trait Section {
    fn name(&self) -> &'static str;

    fn handle_event(&mut self, event: &Event);

    fn update(&mut self);

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
    );
}
```

The application can then maintain registered sections:

```rust
pub struct App {
    pub sections: Vec<Box<dyn Section>>,
    pub active_section: usize,
}
```

This allows additional sections to be introduced without tightly coupling them to the rest of the application.

## Proposed Project Structure

```text
src/
├── main.rs
├── app.rs
├── event.rs
├── config.rs
│
├── database/
│   ├── mod.rs
│   ├── goals.rs
│   ├── study.rs
│   ├── tasks.rs
│   └── habits.rs
│
├── sections/
│   ├── mod.rs
│   ├── dashboard.rs
│   ├── goals.rs
│   ├── study.rs
│   ├── tasks.rs
│   ├── habits.rs
│   └── notes.rs
│
└── widgets/
    ├── mod.rs
    ├── popup.rs
    ├── timer.rs
    └── progress.rs
```

This structure is expected to change as the project grows.

## Data Storage

Persistent application data should use SQLite.

Example location on Linux:

```text
~/.local/share/teverything/teverything.db
```

Configuration should be stored separately.

Example:

```text
~/.config/teverything/config.toml
```

Example database entities may include:

```text
goals
study_sessions
tasks
habits
habit_entries
notes
```

Database access should remain separate from terminal UI logic.

## Navigation

Initial navigation will likely use numbered tabs or keyboard shortcuts.

Example:

```text
[1] Dashboard
[2] Goals
[3] Study
[4] Tasks
[5] Habits
[6] Notes
```

Possible global controls:

```text
1-9       Switch sections
Tab       Next section
Shift+Tab Previous section
Ctrl+P    Command palette
?         Help
q         Quit
```

Individual sections may define additional controls.

## Development Philosophy

Life Terminal should favor simplicity over unnecessary abstraction.

New abstractions should be introduced when there is a clear need for them rather than designing for every possible future feature upfront.

The application should remain usable throughout development.

A simple working implementation is preferred over a complex unfinished architecture.

## Initial Development Milestones

The first milestone should focus only on the application foundation:

1. Create the Rust project.
2. Configure Ratatui and Crossterm.
3. Implement the main application loop.
4. Implement global keyboard handling.
5. Create the section abstraction.
6. Create placeholder sections.
7. Implement tab navigation.
8. Implement clean application shutdown.

The second milestone can introduce SQLite and the first real feature.

Recommended first feature:

**Goals**

After Goals is functional, implement the Study Tracker.

Do not attempt to implement every planned section at once.

## Status

The initial terminal foundation is implemented. It includes modular placeholder
sections for Dashboard, Goals, Study, Tasks, Habits, and Notes, plus keyboard
navigation and clean terminal restoration on exit.

Run it with:

```bash
cargo run
```

Controls:

```text
Tab / Shift+Tab  Change section
1-6              Open a section directly
q                Quit
```
