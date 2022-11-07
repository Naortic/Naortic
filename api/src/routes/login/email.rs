use crate::database::{create, connect, find};

#[get("/email?<email>&<name>&<password>")]
pub async fn email(email: &str, name: &str, password: &str) -> String {

    let (exists, account) = find(email, password);

    if exists {
        return account.unwrap()[0].clone().token;
    }

    let account = create(&mut connect(), name, password, email);

    account.token
}
