use crate::database::update_account_friend_remove;

#[get("/friends/remove?<token>&<friend>")]
pub async fn friends_remove(token: &str, friend: &str) -> String {
    update_account_friend_remove(token, friend).await;

    friend.to_string()
}
