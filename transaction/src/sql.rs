pub const CREATE_TRANSACTION_TABLE: &str = "CREATE TABLE \"transaction\" (\"id\" blob NOT NULL,\"timestamp\" integer NOT NULL,\"inputs\" blob NOT NULL,\"outputs\" blob NOT NULL,PRIMARY KEY (\"id\"))";

pub const QUERY_STATE_BY_ID: &str = "select * from state where id = $1";
