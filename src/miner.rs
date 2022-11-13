use {
    super::schema::miners,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    crate::DBPooledConnection,
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

impl Miner {
    pub fn to_miner_dao(&self) -> MinerDAO {
        MinerDAO {
            id: Uuid::parse_str(self.id.as_str()).unwrap(),
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

// ------------------ POST Reques body for new Miner
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    nickname: String,
}

// ------------------ Data Acess Object (DB Table Records)
#[derive(Queryable, Insertable)]
#[table_name = "miners"]
pub struct MinerDAO {
    pub id: Uuid,
    pub address: Uuid,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl MinerDAO {
    pub fn to_miner(&self, club_name: String) -> Miner {
        Miner {
            id: self.id.to_string(),
            address: self.address.to_string(),
            club_name,
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

pub fn get_club_name(_address: Uuid, conn: &DBPooledConnection) -> String {
    match fetch_wallet_by_id(_address, &conn) {
        Some(matched_wallet) => matched_wallet.club_name,
        None => "Club name not found".to_string(),
    }
}

pub fn fetch_all_miners(conn: &DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;
    match miners.load::<MinerDAO>(conn) {
        Ok(result) => {
            result.into_iter().map(|m| {
                let club_name = get_club_name(m.address, conn);
                m.to_miner(club_name)

            }).collect::<Vec<Miner>>()
        },
        Err(_) => vec![],
    }
}
