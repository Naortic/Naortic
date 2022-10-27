use crate::util::request;

#[get("/github?<code>")]
pub async fn github(code: &str) -> String {
    request(&format!(
        "https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={code}",
        std::env::var("CLIENT_ID").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
    ))
    .await
    .unwrap()
}
