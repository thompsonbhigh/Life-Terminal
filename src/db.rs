use rusqlite::{params, Connection, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

pub struct Goal {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub progress: i64,
    pub subtask_count: i64,
}

pub struct SubTask {
    pub id: i64,
    pub goal_id: i64,
    pub title: String,
    pub completed: bool,
}

pub struct Habit {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub streak: i64,
    pub last_completed: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS tasks (
                id         INTEGER PRIMARY KEY,
                title      TEXT NOT NULL,
                completed  INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS goals (
                id            INTEGER PRIMARY KEY,
                title         TEXT NOT NULL,
                completed     INTEGER NOT NULL DEFAULT 0,
                progress      INTEGER NOT NULL DEFAULT 0,
                subtask_count INTEGER NOT NULL DEFAULT 0,
                created_at    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS goal_subtasks (
                id         INTEGER PRIMARY KEY,
                goal_id    INTEGER NOT NULL,
                title      TEXT NOT NULL,
                completed  INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(goal_id) REFERENCES goals(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS habits (
                id             INTEGER PRIMARY KEY,
                title          TEXT NOT NULL,
                completed      INTEGER NOT NULL DEFAULT 0,
                streak         INTEGER NOT NULL DEFAULT 0,
                last_completed TEXT NOT NULL DEFAULT NEVER,
                created_at     TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            UPDATE habits SET completed = 0 WHERE date(last_completed) < date('now');
            ",
        )?;

        Ok(Self { conn })
    }

    pub fn add_task(&self, title: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO tasks (title) VALUES (?1)", params![title])?;
        Ok(())
    }

    pub fn delete_task(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut statement = self.conn.prepare(
            "
            SELECT id, title, completed
            FROM tasks
            ORDER BY created_at DESC, id DESC
            ",
        )?;

        let tasks = statement
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i64>(2)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }

    pub fn toggle_task(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET completed = NOT completed WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn add_goal(&self, title: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO goals (title) VALUES (?1)", params![title])?;
        Ok(())
    }

    pub fn add_subtask(&self, id: i64, title: &str) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("INSERT INTO goal_subtasks (goal_id, title) VALUES (?1, ?2)", params![id, title])?;
        tx.execute("UPDATE goals SET subtask_count = subtask_count + 1 WHERE id = ?1", params![id])?;
        tx.commit();
        Ok(())
    }

    pub fn delete_goal(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM goals WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_subtask(&self, id: i64, goal_id: i64) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("DELETE FROM goal_subtasks WHERE id = ?1", params![id])?;
        tx.execute("UPDATE goals SET subtask_count = subtask_count - 1 WHERE id = ?1", params![goal_id])?;
        tx.commit();
        Ok(())
    }

    pub fn list_goals(&self) -> Result<Vec<Goal>> {
        let mut statement = self.conn.prepare(
            "
            SELECT id, title, completed, progress, subtask_count
            FROM goals
            ORDER BY created_at DESC, id DESC
            ",
        )?;

        let goals = statement
            .query_map([], |row| {
                Ok(Goal {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i64>(2)? != 0,
                    progress: row.get(3)?,
                    subtask_count: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(goals)
    }

    pub fn list_subtasks(&self, id: i64) -> Result<Vec<SubTask>> {
        let mut statement = self.conn.prepare(
            "
            SELECT id, goal_id, title, completed 
            FROM goal_subtasks
            WHERE goal_id = ?1
            ORDER BY goal_id DESC, created_at DESC
            ",
        )?;

        let subtasks = statement
            .query_map(params![id], |row| {
                Ok(SubTask {
                    id: row.get(0)?,
                    goal_id: row.get(1)?,
                    title: row.get(2)?,
                    completed: row.get::<_, i64>(3)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(subtasks)
    }

    pub fn toggle_goal(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE goals SET completed = NOT completed WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn toggle_subtask(&self, id: i64, goal_id: i64) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?; 
        tx.execute("UPDATE goal_subtasks SET completed = NOT completed WHERE id = ?1", params![id])?;
        tx.execute("UPDATE goals SET subtask_count = (
            SELECT COUNT(*) FROM goal_subtasks WHERE goal_id = ?1
        ),
        progress = (
            SELECT COUNT(*) FROM goal_subtasks WHERE goal_id = ?1 AND completed = 1
        )
        WHERE id = ?1
        ", params![goal_id])?;
        tx.commit();
        Ok(())
    }

    pub fn add_habit(&self, title: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO habits (title) VALUES (?1)", params![title])?;
        Ok(())
    }

    pub fn delete_habit(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM habits WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_habits(&self) -> Result<Vec<Habit>> {
        let mut statement = self.conn.prepare(
            "
            SELECT id, title, completed, streak, last_completed
            FROM habits
            ORDER BY created_at DESC, id DESC
            ",
        )?;

        let habits = statement
            .query_map([], |row| {
                Ok(Habit {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i64>(2)? != 0,
                    streak: row.get(3)?,
                    last_completed: row.get(4).expect("REASON"),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(habits)
    }

    pub fn toggle_habit(&self, id: i64) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("UPDATE habits SET streak = streak + 1 WHERE id = ?1 AND last_completed = DATE('now', '-1 days')", params![id])?;
        tx.execute("UPDATE habits SET streak = 1 WHERE id = ?1 AND (last_completed < DATE('now', '-1 days') OR last_completed = 'NEVER')", params![id])?;
        tx.execute("UPDATE habits SET completed = NOT completed, last_completed = DATE('now') WHERE id = ?1", params![id])?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use rusqlite::Connection;

    fn database() -> Database {
        Database::from_connection(Connection::open_in_memory().unwrap()).unwrap()
    }

    #[test]
    fn tasks_can_be_added_and_completed() {
        let database = database();
        database.add_task("Write a test").unwrap();

        let tasks = database.list_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Write a test");
        assert!(!tasks[0].completed);

        database.toggle_task(tasks[0].id).unwrap();
        assert!(database.list_tasks().unwrap()[0].completed);
    }
}
