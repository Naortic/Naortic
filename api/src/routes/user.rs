use crate::util::request_auth;
use serde_json::Value;

#[get("/name?<token>")]
pub async fn username(token: &str) -> String {
    let json: Value = serde_json::from_str(
        &request_auth("https://api.github.com/user", token)
            .await
            .unwrap(),
    )
    .unwrap();

    json["login"].to_string()
}
