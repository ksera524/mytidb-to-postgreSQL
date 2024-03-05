use diesel::prelude::*;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Queryable, Selectable,Insertable,Debug)]
#[diesel(table_name = crate::schema::stock_prices)]
pub struct StockPrice {
    pub price_id: i32,
    pub stock_symbol: String,
    pub market: String,
    pub date: NaiveDate,
    pub price: Decimal,
    pub volume: i64,
}