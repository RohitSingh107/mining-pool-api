use {
    // crate::DBPool,
    crate::miner::*,
    crate::util::{NotFoundMessage, ResponseType},
    actix_web::web::{Data, Json, Path},
    actix_web::{get, post, HttpResponse},
    uuid::Uuid,
};

// List all miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    // TODO: get all miners obect from db anc convert to miner objects

    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    // TODO: Get the MinerDao object from db where db == requested id and convert to Miner object

    let miner: Option<Miner> = None; // None for now
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Miner not found.".to_string()))
            .get_response(),
    }
}

// Create new miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner() -> HttpResponse {
    // TODO: Create a new MinerDao object from requested inputs and write to db

    let miner: Vec<Miner> = vec![]; // Empty list for now
    ResponseType::Created(miner).get_response()
}
