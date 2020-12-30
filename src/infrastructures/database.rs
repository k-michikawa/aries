use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::io::{Error, ErrorKind};
use std::result::Result;

pub trait DatabaseModule: Sync + Send {
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error>;
}

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn new(database_url: &str, max_size: u32) -> Self {
        // DBのコネクションプールを作成
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(max_size)
            .build(manager)
            .expect("Failed to create pool");
        Database { pool }
    }
}

impl DatabaseModule for Database {
    // FIXME きっと独自エラー作ったほうが良いんだろうなぁ・・・
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        self.pool.get().map_err(|e| Error::new(ErrorKind::Other, e))
    }
}
