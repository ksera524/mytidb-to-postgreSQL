pub mod model;
pub mod schema;

use std::env;
use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use model::StockPrice;
use schema::stock_prices::dsl::*;


fn main() {
    dotenv().ok();
    let connection_mysql = &mut  establish_connection_mysql();
    let connection_pg = &mut establish_connection_pg();

    println!("Connected to MySQL!");

    let results = stock_prices
        .load::<StockPrice>(connection_mysql)
        .expect("Error loading stock prices");

    println!("Displaying {} stock prices", results.len());

    for stock_price in results {
        println!("{:?}", stock_price);
        diesel::insert_into(stock_prices)
            .values(&stock_price)
            .execute(connection_pg)
            .expect("Error saving stock price");
    }
}

fn establish_connection_mysql() -> MysqlConnection {
    let database_url = env::var("MYSQL_DATABASE_URL").expect("MYSQL_DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn establish_connection_pg() -> PgConnection {
    let database_url = env::var("PG_DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}