use crate::database::{create, connect};

#[get("/email?<email>&<name>&<password>")]
pub async fn email(email: &str, name: &str, password: &str) -> String {
    let account = create(&mut connect(), name, password, email);

    account.token
}
