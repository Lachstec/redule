use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

/// A pooled PostgreSQL Connection
/// 
/// establish_connection creates an Instance of DbConn that can be managed by Rocket.
pub type DbConn = Pool<ConnectionManager<PgConnection>>;

/// Establishes a pooled Connection to the database.
/// The URL is fetched from the .env file or the Environment Variables.
/// 
/// Panics if the Pool cannot be created.
pub fn establish_connection() -> DbConn {
    dotenv().ok();
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");
    let manager = ConnectionManager::<PgConnection>::new(&url);
    Pool::new(manager).unwrap()
}