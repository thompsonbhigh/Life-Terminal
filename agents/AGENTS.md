# AGENTS.md

This file contains instructions for AI coding agents working on Life Terminal.

Read this file before making architectural or implementation decisions.

## Project Overview

Life Terminal is a personal productivity TUI written in Rust.

It is intended to act as a terminal-based daily command center containing independent sections for features such as:

* Dashboard
* Goals
* Study tracking
* Tasks
* Habits
* Notes

Additional sections should be easy to add in the future.

The application should feel fast, keyboard-driven, clean, and native to the terminal.

## Core Stack

Use:

* Rust
* Ratatui
* Crossterm
* SQLite
* rusqlite
* serde
* chrono
* TOML

Do not replace major technologies without discussing the architectural reason first.

Do not introduce large dependencies when the required behavior can reasonably be implemented with the existing stack.

## Primary Architecture Rule

Major application features should be implemented as independent sections.

The architecture should support something conceptually similar to:

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

The exact trait may evolve as the application's needs become clearer.

Do not force abstractions purely to match this example.

The important requirement is that sections remain modular and do not become tightly coupled to one another.

## Separation of Concerns

Keep the following concerns separate whenever practical:

```text
UI rendering
Application state
Input handling
Database access
Configuration
Business logic
```

For example, a Ratatui widget should not directly contain SQL queries.

Prefer:

```text
UI
 ↓
Section / application logic
 ↓
Repository / database layer
 ↓
SQLite
```

## Sections

Each section should normally live in:

```text
src/sections/
```

Examples:

```text
dashboard.rs
goals.rs
study.rs
tasks.rs
habits.rs
notes.rs
```

A new section should not require major modifications across unrelated sections.

Ideally, adding a section should involve:

1. Implementing the section.
2. Registering it with the application.
3. Adding its navigation entry.

## Shared Widgets

Reusable terminal UI components belong in:

```text
src/widgets/
```

Examples:

```text
popup.rs
timer.rs
progress.rs
confirm.rs
input.rs
```

Do not duplicate substantial UI logic across sections when it can reasonably become a reusable widget.

Do not prematurely generalize tiny pieces of UI.

## Database

Use SQLite for persistent user data.

Database code should live under:

```text
src/database/
```

Prefer small domain-specific database modules.

Example:

```text
database/
├── mod.rs
├── goals.rs
├── study.rs
├── tasks.rs
└── habits.rs
```

Do not scatter SQL queries throughout UI files.

Database operations should return useful domain data rather than terminal-specific representations.

## Configuration

Application configuration should use TOML where appropriate.

Configuration includes things such as:

* Keybindings
* UI preferences
* Default section
* Timer preferences
* Future customization options

Do not store normal user-generated application data in the config file.

Persistent user data belongs in SQLite.

## Input Handling

Distinguish between global input and section-specific input.

Examples of global input:

```text
q
Ctrl+P
Tab
Shift+Tab
1-9
?
```

Examples of section-specific input:

```text
a    Add item
d    Delete item
e    Edit item
Enter Select item
Space Toggle completion
```

Avoid creating one giant keyboard event handler containing every command in the application.

## Application State

Keep application state explicit.

Avoid unnecessary global mutable state.

The application should make it easy to understand:

* Which section is active
* Whether a popup is open
* Whether text input is active
* Whether a timer is running
* Which item is selected

Use enums when they make state transitions clearer.

For example:

```rust
enum InputMode {
    Normal,
    Editing,
}
```

or:

```rust
enum Overlay {
    None,
    CommandPalette,
    Help,
    Confirmation,
}
```

## UI Guidelines

Favor clean terminal interfaces over excessive decoration.

Use:

* Consistent spacing
* Clear selected states
* Helpful borders when needed
* Minimal color
* Visible keyboard shortcuts
* Responsive layouts

Avoid filling every area with boxes simply because Ratatui supports borders.

Important information should be visually prominent.

Secondary information should remain subtle.

## Keyboard-First Design

Every important operation should be possible without a mouse.

Prefer fast keyboard workflows.

Examples:

```text
a        Add
e        Edit
d        Delete
Space    Toggle
Enter    Open/select
Esc      Cancel/go back
Ctrl+P   Command palette
?        Help
```

Exact shortcuts may change, but similar actions should use consistent shortcuts throughout the application.

## Command Palette

The architecture should allow a future command palette.

Possible commands include:

```text
:goal Finish assignment
:study start CSCE313
:study stop
:task Buy groceries
:open study
```

Do not implement a full command language until it becomes useful.

However, avoid architectural decisions that would make global commands unnecessarily difficult later.

## Error Handling

Do not use `unwrap()` throughout production application logic.

Using `unwrap()` may be acceptable in tests or situations where failure is provably impossible and clearly documented.

Prefer meaningful propagation with:

```rust
Result<T, E>
```

Potential error libraries:

```text
anyhow
thiserror
```

Introduce them only when useful.

Errors shown to users should be understandable.

Avoid exposing raw SQLite or Rust implementation errors directly in the UI when a clearer message can be provided.

## Rust Style

Follow idiomatic Rust.

Run:

```bash
cargo fmt
```

and:

```bash
cargo clippy
```

before considering a substantial change complete.

Prefer readable code over clever code.

Use descriptive names.

Avoid unnecessary cloning.

Avoid fighting the borrow checker with excessive shared mutable ownership unless the architecture genuinely requires it.

Do not introduce `Rc<RefCell<_>>` or `Arc<Mutex<_>>` simply to avoid designing clear ownership.

## Async Code

Do not introduce Tokio or asynchronous architecture unless there is a real asynchronous requirement.

The initial application does not require async code.

Possible future reasons for async include:

* Network APIs
* Background synchronization
* Integrations
* Long-running external processes

A study timer itself does not require an asynchronous runtime.

## Dependencies

Before adding a crate:

1. Determine whether the standard library or existing dependencies can reasonably solve the problem.
2. Prefer established crates.
3. Avoid adding multiple crates that solve the same problem.
4. Keep the dependency tree reasonable.

Do not add a dependency solely to save a few lines of straightforward code.

## Testing

Business logic should be testable without rendering an actual terminal whenever possible.

Good candidates for tests include:

* Goal completion logic
* Study duration calculations
* Date grouping
* Habit streaks
* Database queries
* Commands
* State transitions

UI snapshot testing may be added later if it becomes valuable.

## Scope Control

Do not implement unrelated future features while completing a focused task.

For example, when implementing Goals:

Do not also build:

* Calendar integrations
* Workout tracking
* Finance tracking
* Plugin systems
* Online synchronization

unless explicitly requested.

Build features incrementally.

## Current Development Order

Unless instructed otherwise, prioritize development approximately as follows:

1. Application skeleton
2. Ratatui initialization
3. Event loop
4. Global navigation
5. Section architecture
6. Placeholder sections
7. Goals
8. SQLite persistence
9. Study tracker
10. Dashboard
11. Tasks
12. Habits
13. Notes
14. Command palette

This order is guidance rather than a strict requirement.

## First Implementation Target

The initial version should open into a functional terminal UI resembling:

```text
┌──────────────────────────────────────────────────────┐
│ Life Terminal                                         │
├──────────────────────────────────────────────────────┤
│ Dashboard   Goals   Study   Tasks   Habits   Notes   │
├──────────────────────────────────────────────────────┤
│                                                      │
│                  Dashboard                           │
│                                                      │
│              Section coming soon                    │
│                                                      │
├──────────────────────────────────────────────────────┤
│ Tab: Next   Shift+Tab: Previous   q: Quit            │
└──────────────────────────────────────────────────────┘
```

The first implementation should prove:

* Terminal initialization works
* Rendering works
* Keyboard input works
* Navigation works
* Sections are modular
* Terminal state restores correctly after exiting

Do not implement SQLite or real productivity functionality until this foundation works unless explicitly requested.

## Terminal Safety

Always restore the terminal correctly when the program exits or encounters an error.

Pay particular attention to:

* Raw mode
* Alternate screen
* Cursor visibility

A crash should not leave the user's terminal unusable.

## Documentation

Update the README when introducing major user-facing functionality or significant architectural changes.

Comments should explain **why**, not merely restate what the code does.

Avoid excessive comments on self-explanatory Rust.

## General Principle

Prefer the simplest architecture that supports the application's current requirements while keeping sections modular enough for future expansion.

Do not overengineer Life Terminal into a framework.

It is first and foremost a useful personal application.
