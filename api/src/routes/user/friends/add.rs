use crate::database::update_account_friend_add;

#[get("/friends/add?<token>&<friend>")]
pub async fn friends_add(token: &str, friend: &str) -> String {
    update_account_friend_add(token, friend).await;

    friend.to_string()
}
