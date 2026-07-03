use crate::sqlite::*;

impl InternalDB {
    pub fn new(path: String) -> Result<Self, rusqlite::Error> {
        let value = Self {path};
        value.init()?;

        return Ok(value);
    }

    pub fn open(&self) -> Result<Connection, rusqlite::Error> {
        return Ok(Connection::open(self.path.clone())?);
    }

    fn init(&self) -> Result<(), rusqlite::Error> {
        let _changed = self.open()?.execute_batch("
            CREATE TABLE IF NOT EXISTS domain (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS artist (
                id TEXT NOT NULL,
                domain_id INTEGER NOT NULL,
                json TEXT NOT NULL,

                FOREIGN KEY (domain_id) REFERENCES domain(id)
            );"
        )?;

        return Ok(());
    }

    pub fn add_domain(&self, domain: String) -> Result<i64, rusqlite::Error> {
        let id = self.open()?.query_row("
                INSERT INTO domain (domain)
                VALUES (?)
                ON CONFLICT(domain)
                DO UPDATE SET domain = excluded.domain
                RETURNING id
            ",
            [domain],
            |row| {row.get::<usize, i64>(0)}
        );

        return id;
    }

    pub fn add_artist(&self, domain_id: i64, artist_id: uuid::Uuid, json: serde_json::Value) -> Result<(), rusqlite::Error> {
        let _changed = self.open()?.execute("
                INSERT INTO artist (id, domain_id, json)
                VALUES (?, ?, ?)
            ",
            [artist_id.to_string(), domain_id.to_string(), json.to_string()]
        )?;

        return Ok(());
    }

    pub fn get_artist(&self, domain_id: i64, artist_id: uuid::Uuid) -> Result<Option<String>, rusqlite::Error> {
        let artist = self.open()?.query_row("
                SELECT json FROM artist
                WHERE domain_id=? AND id=?
                LIMIT 1
            ",
            [domain_id.to_string(), artist_id.to_string()],
            |row| row.get::<usize, String>(0)
        );

        let artist = match artist {
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => {return Ok(None)},
                _ => Err(e)?
            }
            Ok(v) => v
        };

        return Ok(Some(artist));
    }
}
