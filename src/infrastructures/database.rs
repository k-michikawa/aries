use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::result::Result;

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

    // FIXME きっと独自エラー作ったほうが良いんだろうなぁ・・・
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, std::io::Error> {
        self.pool
            .get()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}
