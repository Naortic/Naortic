pub mod add;
pub mod remove;

use crate::database::read_account;
use rocket::serde::json::Json;

#[get("/friends?<token>")]
pub async fn friends(token: &str) -> Json<Vec<String>>{
    let friends = read_account(token).await[0].get(0);

    Json(friends)
}
