use rusqlite::{Connection, OpenFlags, Result};

const INSERT_QUERY: &str =
    "INSERT INTO games (width, height, rounds, data) VALUES (?1, ?2, ?3, ?4)";
const CREATE_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS games (
    id   INTEGER PRIMARY KEY,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    rounds INTEGER NOT NULL,
    data BLOB
)";

#[derive(Debug)]
pub(crate) struct DatabaseGameState {
    /// database id
    pub(crate) id: Option<i32>,
    /// width of the game
    pub(crate) width: usize,
    /// height of the game
    pub(crate) height: usize,
    /// number of rounds necessary to replay the saved state
    pub(crate) rounds: usize,
    /// game state of the game
    pub(crate) data: Vec<u8>,
}

impl DatabaseGameState {
    pub(crate) fn new(
        width: usize,
        height: usize,
        rounds: usize,
        data: Vec<u8>,
    ) -> DatabaseGameState {
        DatabaseGameState {
            id: None,
            width,
            height,
            rounds,
            data,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DatabaseConnection {
    pub(crate) conn: Connection,
    pub(crate) path: String,
}

impl DatabaseConnection {
    pub(crate) fn new(path: &str) -> DatabaseConnection {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .expect("failed to open database or database is not present");
        DatabaseConnection {
            conn,
            path: path.to_string(),
        }
    }
    pub(crate) fn create_tables(&self) {
        self.conn.execute(CREATE_TABLE_QUERY, ()).unwrap();
    }

    pub(crate) fn insert_game(&self, game: DatabaseGameState) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            INSERT_QUERY,
            (game.width, game.height, game.rounds, game.data),
        )
    }

    pub(crate) fn list_games(&self) -> Vec<DatabaseGameState> {
        // Todo: catch this panic
        let mut stmt = self
            .conn
            .prepare("SELECT id, width, height, rounds, data FROM games")
            .unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(DatabaseGameState {
                    id: Some(row.get(0).unwrap()),
                    width: row.get(1).unwrap(),
                    height: row.get(2).unwrap(),
                    rounds: row.get(3).unwrap(),
                    data: row.get(4).unwrap(),
                })
            })
            .unwrap();
        person_iter
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<DatabaseGameState>>()
    }

    pub(crate) fn get_game_by_idx(&self, get: usize) -> DatabaseGameState {
        let query = format!(
            "SELECT id, width, height, rounds, data FROM games WHERE id={}",
            get
        );
        let mut stmt = self.conn.prepare(&query).unwrap();
        let mut person_iter = stmt
            .query_map([], |row| {
                Ok(DatabaseGameState {
                    id: Some(row.get(0).unwrap()),
                    width: row.get(1).unwrap(),
                    height: row.get(2).unwrap(),
                    rounds: row.get(3).unwrap(),
                    data: row.get(4).unwrap(),
                })
            })
            .unwrap();
        dbg!("after stmt");
    }
}
