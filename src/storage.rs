use std::{error::Error, path::PathBuf};

use rusqlite::{params, Connection};
use sql_builder::SqlBuilder;

pub struct LeaderboardStorage {
    conn: Connection,
}

impl LeaderboardStorage {
    pub fn open(path: PathBuf) -> Self {
        Self {
            conn: Connection::open(path).unwrap(),
        }
    }

    pub fn open_in_memory() -> Self {
        Self {
            conn: Connection::open_in_memory().unwrap(),
        }
    }

    pub fn init_schema(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
                    CREATE TABLE IF NOT EXISTS leaderboard (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        nickname TEXT NOT NULL,
                        score INTEGER NOT NULL,
                        last_updated TEXT NOT NULL
                    )"#,
        )?;

        stmt.execute([])?;

        Ok(())
    }

    pub fn delete_schema(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.conn.prepare("DROP TABLE IF EXISTS leaderboard")?;

        stmt.execute([])?;

        Ok(())
    }

    pub fn insert_or_update(&mut self, new_score: usize) -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("leaderboard")
            .field("id")
            .and_where("nickname = 'You'")
            .sql()?;
        let mut stmt = self.conn.prepare(&sql)?;

        let mut rows = stmt.query([])?;

        if let Ok(Some(row)) = rows.next() {
            let id = row.get::<_, usize>(0)?;
            let sql = SqlBuilder::update_table("leaderboard")
                .set("last_updated", "date('now')")
                .set("score", "?1")
                .and_where("id = ?2")
                .sql()?;

            self.conn.execute(&sql, [new_score, id])?;

            Ok(())
        } else {
            let sql = SqlBuilder::insert_into("leaderboard")
                .fields(&["nickname, score, last_updated"])
                .values(&["?1", "?2", "date('now')"])
                .sql()?;

            self.conn.execute(&sql, params!["You", new_score])?;

            Ok(())
        }
    }

    pub fn select_best_score(&mut self) -> Result<(String, usize), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("leaderboard")
            .fields(&["nickname", "score"])
            .order_desc("score")
            .limit(1)
            .sql()?;

        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            let name = row.get::<_, String>(0)?;
            let score = row.get::<_, usize>(1)?;

            return Ok((name, score));
        }

        Err("no rows found".into())
    }
}
