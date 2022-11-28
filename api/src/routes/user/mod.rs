pub mod friends;
pub mod name;

use crate::{database::read_account, account::Account};
use rocket::serde::json::Json;

#[get("/?<token>")]
pub async fn index(token: &str) -> Json<Account> {
    let row = &read_account(token).await[0];
    
    let account = Account {
        name: row.get(0),
        email: row.get(1),
        friends: row.get(2),
        token: token.to_string(),
    };

    Json(account)
}
