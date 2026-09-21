use rusqlite::{params, Connection, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
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
