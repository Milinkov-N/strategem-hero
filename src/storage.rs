use std::{error::Error, path::PathBuf};

use rusqlite::{params, Connection, Row};
use sql_builder::SqlBuilder;

pub struct Record {
    pub id: usize,
    pub nickname: String,
    pub score: usize,
    pub last_updated: String,
}

impl TryFrom<&Row<'_>> for Record {
    type Error = rusqlite::Error;
    fn try_from(value: &Row) -> Result<Self, Self::Error> {
        let id = value.get::<_, usize>(0)?;
        let nickname = value.get::<_, String>(1)?;
        let score = value.get::<_, usize>(2)?;
        let last_updated = value.get::<_, String>(3)?;

        Ok(Self {
            id,
            nickname,
            score,
            last_updated,
        })
    }
}

pub struct LeaderboardStorage {
    conn: Connection,
}

impl LeaderboardStorage {
    const TABLE_NAME: &str = "leaderboard";
    const TABLE_FIELDS: &'static [&'static str] = &["id", "nickname", "score", "last_updated"];

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

    pub fn seed_schema(&mut self) -> Result<(), Box<dyn Error>> {
        if self.count()? > 0 {
            return Ok(());
        }

        let now = "date('now')";
        let sql = SqlBuilder::insert_into(Self::TABLE_NAME)
            .fields(&["nickname", "score", "last_updated"])
            .values(&["'John Helldiver'", "20000", now])
            .values(&["'Eagle-1'", "14500", now])
            .values(&["'Pelican-1'", "11200", now])
            .values(&["'Democracy Officer'", "8300", now])
            .values(&["'You'", "0", now])
            .sql()?;

        self.conn.execute(&sql, [])?;

        Ok(())
    }

    pub fn drop_schema(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .conn
            .prepare(&format!("DROP TABLE IF EXISTS {}", Self::TABLE_NAME))?;

        stmt.execute([])?;

        Ok(())
    }

    pub fn count(&mut self) -> Result<usize, Box<dyn Error>> {
        let sql = SqlBuilder::select_from(Self::TABLE_NAME)
            .field("COUNT(*)")
            .sql()?;
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        if let Ok(Some(row)) = rows.next() {
            return Ok(row.get::<_, usize>(0)?);
        }

        Ok(0)
    }

    pub fn select_all(&mut self) -> Result<Vec<Record>, Box<dyn Error>> {
        let mut records = vec![];
        let sql = SqlBuilder::select_from(Self::TABLE_NAME)
            .fields(Self::TABLE_FIELDS)
            .order_desc("score")
            .sql()?;

        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            records.push(Record::try_from(row)?);
        }

        Ok(records)
    }

    pub fn insert_or_update(
        &mut self,
        nickname: &str,
        new_score: usize,
    ) -> Result<(), Box<dyn Error>> {
        let record = self.find_by_name(nickname);

        if let Some(record) = record {
            let id = record.id;
            let sql = SqlBuilder::update_table(Self::TABLE_NAME)
                .set("last_updated", "date('now')")
                .set("score", "?1")
                .and_where("id = ?2")
                .sql()?;

            self.conn.execute(&sql, [new_score, id])?;

            Ok(())
        } else {
            let sql = SqlBuilder::insert_into(Self::TABLE_NAME)
                .fields(&["nickname, score, last_updated"])
                .values(&["?1", "?2", "date('now')"])
                .sql()?;

            self.conn.execute(&sql, params![nickname, new_score])?;

            Ok(())
        }
    }

    pub fn find_by_name(&mut self, nickname: &str) -> Option<Record> {
        let sql = SqlBuilder::select_from(Self::TABLE_NAME)
            .fields(Self::TABLE_FIELDS)
            .and_where("nickname = ?1")
            .sql()
            .ok()?;
        let mut stmt = self.conn.prepare(&sql).ok()?;
        let mut rows = stmt.query([nickname]).ok()?;

        match rows.next() {
            Ok(Some(row)) => Some(Record::try_from(row).ok()?),
            _ => None,
        }
    }
}
