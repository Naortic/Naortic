use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

use crate::database::create_account;

#[get("/email?<email>&<name>&<password>")]
pub async fn email(email: &str, name: &str, password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(name);
    hasher.update(password);

    let hash = hasher.finalize();
    let token = &Base64::encode_string(&hash);

    let account = create_account(name, password, email, token).await;

    account.get(0).unwrap().get(0)
}
