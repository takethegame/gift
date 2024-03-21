use rocket_sync_db_pools::{database, diesel};

#[cfg(any(
    feature = "diesel_sqlite_pool",
    feature = "diesel_postgres_pool",
    feature = "diesel_mysql_pool"
))]
pub use diesel;

#[database("lottery")]
pub struct DBConn(diesel::MysqlConnection);

