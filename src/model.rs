use diesel::prelude::*;
use chrono::NaiveDate;

#[derive(Queryable, Selectable,Insertable,Debug)]
#[diesel(table_name = crate::schema::stock_prices)]
pub struct StockPrice {
    pub price_id: i32,
    pub stock_symbol: String,
    pub market: String,
    pub date: NaiveDate,
    pub price: f32,
    pub volume: i64,
}