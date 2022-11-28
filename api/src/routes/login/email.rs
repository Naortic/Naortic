use crate::database::find_account;
#[get("/email?<email>&<password>")]
pub async fn email(email: &str, password: &str) -> String {
    let account = find_account(email, password).await;

    if account.is_err() {
        String::from("account not found")
    } else {
        let account = account.unwrap();
        account[0].get(0)
    }
}
