use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn gen_pool() -> Pool<MySql> {
    let database_url = std::env::var("DATABASE_URL").expect("Missing Env: DATABASE_URL");
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            eprintln!("[DEBUG] Connection to the database is successful!");
            pool
        }
        Err(err) => {
            panic!("[ERROR] Failed to connect to the database: {:?}", err);
        }
    }
}