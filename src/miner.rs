use {
    super::schema::miners,
    crate::wallet::*,
    crate::DBPooledConnection,
    diesel::result::Error,
    diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl},
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

// --------------- JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32, // MH/s
    pub shares_mined: i32,
}
