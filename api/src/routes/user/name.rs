use crate::database::get;

#[get("/name?<token>")]
pub async fn name(token: &str)-> String {
    let account = get(token);

    account.name
}