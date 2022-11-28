use crate::database::read_account;

#[get("/name?<token>")]
pub async fn name(token: &str) -> String {
    let account = read_account(token).await;

    account[0].get(0)
}
