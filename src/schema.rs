// @generated automatically by Diesel CLI.

diesel::table! {
    stock_prices (price_id) {
        price_id -> Integer,
        #[max_length = 255]
        stock_symbol -> Varchar,
        #[max_length = 255]
        market -> Varchar,
        date -> Date,
        price -> Float,
        volume -> Bigint,
    }
}
