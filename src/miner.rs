use {
    serde::{Deserialize, Serialize},
};


// ------------------ JSON Payload (Rest)
#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}


// ------------------ POST Reques body for new Miner
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    nickname: String,
}

// ------------------ Data Acess Object (DB Table Records)
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}


