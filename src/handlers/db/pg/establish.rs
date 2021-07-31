







use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::env;
use dotenv::dotenv;




pub type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;
pub async fn connection(db_name: &str) -> Result<Pool, Box<dyn std::error::Error>>{

    dotenv().expect("⚠️ .env file not found");
    let pghost = env::var("PGHOST").expect("⚠️ couldn't find the postgres host");
    let pgport = env::var("PGPORT").expect("⚠️ couldn't find the postgres port");
    let pg_user = env::var("POSTGRES_USER").expect("⚠️ couldn't find the postgres inobi user");
    let pg_password = env::var("POSTGRES_PASSWORD").expect("⚠️ couldn't find the postgres inobi password");
    let db_url = format!("user={} password={} host={} port={} dbname={}", pg_user, pg_password, pghost, pgport, db_name).parse().expect("⚠️ pg db url config error"); //-- unwrap() will make the compiler panic on any error  
    let manager = PostgresConnectionManager::new(db_url, NoTls);
    match r2d2::Pool::new(manager){
        Ok(pool) => {
            Ok(pool) //-- it's a pool object of connections
        },
        Err(e) => {
            eprintln!("⚠️ failed to create pg pool");
            Err(From::from(e))
        }
    } 
}
