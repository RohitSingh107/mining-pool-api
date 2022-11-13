use {
    // crate::DBPool,
    crate::miner::*,
    crate::wallet::*,
    crate::util::{NotFoundMessage, ResponseType},
    actix_web::web::{Data, Json, Path},
    actix_web::{get, post, HttpResponse},
    uuid::Uuid,
};

// List all miners
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    // TODO: get all wallets obect from db anc convert to miner objects

    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    // TODO: Get the WalletDao object from db where db == requested id and convert to Miner object

    let wallet: Option<Wallet> = None; // None for now
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet not found.".to_string()))
            .get_response(),
    }
}

// Create new miner
#[post("/wallets")]
pub async fn create_wallet() -> HttpResponse {
    // TODO: Create a new WalletDao object from requested inputs and write to db

    let wallet: Vec<Wallet> = vec![]; // Empty list for now
    ResponseType::Created(wallet).get_response()
}
